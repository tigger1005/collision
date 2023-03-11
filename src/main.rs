use nannou::prelude::*;

const RADIUS: f32 = 8.0;
const DIAMETER: f32 = 2.0 * RADIUS;
const GRID_SIZE: usize = 80 as usize;

fn main() {
    nannou::app(model).update(update).run();
}

struct Grid {
    data: [[Vec<usize>; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    fn new() -> Self {
        const IN_1: Vec<usize> = Vec::new();
        const IN_2: [Vec<usize>; GRID_SIZE] = [IN_1; GRID_SIZE];
        Grid {
            data: [IN_2; GRID_SIZE],
        }
    }
    fn get(&self, x: usize, y: usize) -> &Vec<usize> {
        &self.data[x][y]
    }

    fn clear(&mut self) {
        self.data
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|element| element.clear()));
    }

    fn add(&mut self, point: &Point2, index: usize) {
        let x: usize =
            ((point.x + (DIAMETER * GRID_SIZE as f32 / 2.0)) / DIAMETER) as usize % GRID_SIZE;
        let y: usize =
            ((point.y + (DIAMETER * GRID_SIZE as f32 / 2.0)) / DIAMETER) as usize % GRID_SIZE;
        self.data[x][y].push(index);
    }
}

struct Model {
    elements: Vec<Point2>,
    grid: Grid,
}

impl Model {
    fn update_grid(&mut self) {
        self.grid.clear();
        self.elements
            .iter()
            .enumerate()
            .for_each(|(index, element)| self.grid.add(element, index))
    }

    fn find_collision_grid(&mut self) {
        for x in 1..(GRID_SIZE - 1) {
            for y in 1..(GRID_SIZE - 1) {
                let current_cell = self.grid.get(x, y).clone();
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let other_cell = self
                            .grid
                            .get((x as isize + dx) as usize, (y as isize + dy) as usize)
                            .clone();
                        self.check_cell_collisions(&current_cell, &other_cell);
                    }
                }
            }
        }
    }

    fn check_cell_collisions(&mut self, cell_1: &Vec<usize>, cell_2: &Vec<usize>) {
        for obj_index_1 in cell_1 {
            for obj_index_2 in cell_2 {
                if *obj_index_1 != *obj_index_2 && self.cell_collision(*obj_index_1, *obj_index_2) {
                    self.solve_collision(*obj_index_1, *obj_index_2)
                }
            }
        }
    }

    fn cell_collision(&self, obj_index_1: usize, obj_index_2: usize) -> bool {
        let d = self.elements[obj_index_1].distance(self.elements[obj_index_2]);
        d < DIAMETER
    }

    fn solve_collision(&mut self, obj_index_1: usize, obj_index_2: usize) {
        let d = self.elements[obj_index_1].distance(self.elements[obj_index_2]);
        let t = (RADIUS + (d * 0.5)) / d;
        let p2 = self.elements[obj_index_2].clone();
        self.elements[obj_index_2].x = (1.0 - t) * self.elements[obj_index_1].x + t * p2.x;
        self.elements[obj_index_2].y = (1.0 - t) * self.elements[obj_index_1].y + t * p2.y;
        self.elements[obj_index_1].x = (1.0 - t) * p2.x + t * self.elements[obj_index_1].x;
        self.elements[obj_index_1].y = (1.0 - t) * p2.y + t * self.elements[obj_index_1].y;
    }

    fn add(&mut self, point: &Point2) {
        self.grid.add(point, self.elements.len());
        self.elements.push(*point);
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(
            DIAMETER as u32 * GRID_SIZE as u32,
            DIAMETER as u32 * GRID_SIZE as u32,
        )
        .view(view)
        .event(event)
        .build()
        .unwrap();

    Model {
        elements: Vec::new(),
        grid: Grid::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update_grid();
    model.find_collision_grid();
}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(_) => {
            //        MousePressed(_button) => {
            let pos = app.mouse.position();
            let point = Point2::new(pos.x, pos.y);
            m.add(&point);
        }
        _other => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse with default size and position.

    for (i, element) in model.elements.iter().enumerate() {
        draw.ellipse()
            .color(STEELBLUE)
            .stroke(BLACK)
            .stroke_weight(1.0)
            .xy(*element)
            .radius(RADIUS)
            .color(Rgb::<u8>::new(
                ((i as f32 * 1.1) as usize % 256) as u8,
                ((i as f32 * 1.3) as usize % 256) as u8,
                ((i as f32 * 1.5) as usize % 256) as u8,
            ));
        //        draw.text(i.to_string().as_str()).xy(*element);
    }
    let text = format!("Elements: {}", model.elements.len());
    let x = app.window_rect().top_left().x + 50.0;
    let y = app.window_rect().top_left().y - 10.0;
    draw.text(text.as_str()).x_y(x, y);
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
