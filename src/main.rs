use tetra::graphics::{self, Color, Rectangle, Texture, Text, Font};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::audio::{self, Sound};
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 12.0;
const BALL_SPEED: f32 = 9.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    points: f32,
}

impl Entity {
    // fn new(texture: Texture, position: Vec2<f32>, points: f32) -> Entity {
    //     Entity::with_velocity(texture, position, Vec2::zero(), points)
    // }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>, points: f32) -> Entity {
        Entity {
            texture,
            position,
            velocity,
            points,
        }
    }

    fn with_points(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>, points: f32) -> Entity {
        Entity {
            texture,
            position,
            velocity,
            points,
        }
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    game_started: bool,
    game_ended: bool,
    end_text: String,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );
        let player1_points: f32 = 0.0;

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );
        let player2_points: f32 = 0.0;

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);
        let ball_points = 0.0;

        Ok(GameState {
            player1: Entity::with_points(player1_texture, player1_position, Vec2::zero(), player1_points),
            player2: Entity::with_points(player2_texture, player2_position, Vec2::zero(), player2_points),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity, ball_points),
            game_started: false,
            game_ended: false,
            end_text: String::from(""),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        // Start game while in the main menu and reset all game values to their original values
        if input::is_key_pressed(ctx, Key::Space) && self.game_started == false && self.game_ended == false {
            self.game_started = true;
            self.game_ended = false;

            self.player1.position.x = 16.0;
            self.player1.position.y = (WINDOW_HEIGHT - self.player1.texture.height() as f32) / 2.0;

            self.player2.position.x = WINDOW_WIDTH - self.player2.texture.width() as f32 - 16.0;
            self.player2.position.y = (WINDOW_HEIGHT - self.player2.texture.height() as f32) / 2.0;

            self.ball.position.x = WINDOW_WIDTH / 2.0 - self.ball.texture.width() as f32 / 2.0;
            self.ball.position.y = WINDOW_HEIGHT / 2.0 - self.ball.texture.height() as f32 / 2.0;
            self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
        }

        // Restart game while in restart menu and reset all game values to their original value
        if input::is_key_pressed(ctx, Key::R) && self.game_started == false && self.game_ended == true {
            self.game_started = true;
            self.game_ended = false;

            self.player1.position.x = 16.0;
            self.player1.position.y = (WINDOW_HEIGHT - self.player1.texture.height() as f32) / 2.0;
            self.player1.points = 0.0;

            self.player2.position.x = WINDOW_WIDTH - self.player2.texture.width() as f32 - 16.0;
            self.player2.position.y = (WINDOW_HEIGHT - self.player2.texture.height() as f32) / 2.0;
            self.player2.points = 0.0;

            self.ball.position.x = WINDOW_WIDTH / 2.0 - self.ball.texture.width() as f32 / 2.0;
            self.ball.position.y = WINDOW_HEIGHT / 2.0 - self.ball.texture.height() as f32 / 2.0;
            self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
        }

        // Go to menu on keypress M while in restart menu
        if input::is_key_pressed(ctx, Key::M) && self.game_started == false && self.game_ended == true {
            self.game_started = false;
            self.game_ended = false;
        }

        // Exit game on key press Q while in main menu
        if input::is_key_pressed(ctx, Key::Q) && self.game_started == false && self.game_ended == false {
            window::quit(ctx);
        }

        // Game logic
        if self.game_started == true && self.game_ended == false {

            if input::is_key_down(ctx, Key::W) {
                if self.player1.position.y > WINDOW_HEIGHT - WINDOW_HEIGHT {
                    self.player1.position.y -= PADDLE_SPEED;
                }
            }

            if input::is_key_down(ctx, Key::S) {
                if (self.player1.position.y + self.player1.texture.height() as f32) < WINDOW_HEIGHT {
                    self.player1.position.y += PADDLE_SPEED;
                }
            }

            if input::is_key_down(ctx, Key::Up) {
                if self.player2.position.y > WINDOW_HEIGHT - WINDOW_HEIGHT {
                    self.player2.position.y -= PADDLE_SPEED;
                }
            }

            if input::is_key_down(ctx, Key::Down) {
                if (self.player2.position.y + self.player2.texture.height() as f32) < WINDOW_HEIGHT {
                    self.player2.position.y += PADDLE_SPEED;
                }
            }

            self.ball.position += self.ball.velocity;

            let player1_bounds = self.player1.bounds();
            let player2_bounds = self.player2.bounds();
            let ball_bounds = self.ball.bounds();

            let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
                Some(&self.player1)
            } else if ball_bounds.intersects(&player2_bounds) {
                Some(&self.player2)
            } else {
                None
            };

            if let Some(paddle) = paddle_hit {
                audio::set_master_volume(ctx, 0.4);
                let hit_sound = Sound::from_file_data(include_bytes!("../resources/hit.mp3"));
                hit_sound.play(ctx)?;

                // Increase the ball's velocity, then flip it.
                self.ball.velocity.x = -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));
                
                // Calculate the offset between the paddle and the ball, as a number between
                // -1.0 and 1.0
                let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

                // Apply the spin to the ball.
                self.ball.velocity.y += PADDLE_SPIN * -offset;
            }

            if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
                self.ball.velocity.y = -self.ball.velocity.y;
            }

            if self.ball.position.x < 0.0 {
                // window::quit(ctx);

                // Reset all game values back to default except for points
                self.player1.position.x = 16.0;
                self.player1.position.y = (WINDOW_HEIGHT - self.player1.texture.height() as f32) / 2.0;
    
                self.player2.position.x = WINDOW_WIDTH - self.player2.texture.width() as f32 - 16.0;
                self.player2.position.y = (WINDOW_HEIGHT - self.player2.texture.height() as f32) / 2.0;
    
                self.ball.position.x = WINDOW_WIDTH / 2.0 - self.ball.texture.width() as f32 / 2.0;
                self.ball.position.y = WINDOW_HEIGHT / 2.0 - self.ball.texture.height() as f32 / 2.0;
                self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);

                // Increment points of player 2
                self.player2.points += 1.0;
            }

            if self.ball.position.x > WINDOW_WIDTH {
                // window::quit(ctx);

                // Reset all game values back to default except for points
                self.player1.position.x = 16.0;
                self.player1.position.y = (WINDOW_HEIGHT - self.player1.texture.height() as f32) / 2.0;
    
                self.player2.position.x = WINDOW_WIDTH - self.player2.texture.width() as f32 - 16.0;
                self.player2.position.y = (WINDOW_HEIGHT - self.player2.texture.height() as f32) / 2.0;
    
                self.ball.position.x = WINDOW_WIDTH / 2.0 - self.ball.texture.width() as f32 / 2.0;
                self.ball.position.y = WINDOW_HEIGHT / 2.0 - self.ball.texture.height() as f32 / 2.0;
                self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);

                // Increment points for player 1
                self.player1.points += 1.0;
            }

            if self.player1.points == 11.0 {
                self.game_ended = true;
                self.game_started = false;

                self.end_text = String::from("Player 1 wins");
            } else if self.player2.points == 11.0 {
                self.game_ended = true;
                self.game_started = false;

                self.end_text = String::from("Player 2 wins");
            }
        }

        // End function return
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // 0.392, 0.584, 0.929
        graphics::clear(ctx, Color::rgb(0.080, 0.080, 0.080));

        // Restart Menu
        if self.game_started == false && self.game_ended == true {
            let title_font = Font::from_file_data(ctx,include_bytes!("../resources/RUBBBB__.TTF"));
            let title_text = Text::new(&self.end_text, title_font, 40.0);
            let text_position = Vec2::new(
                (WINDOW_WIDTH / 2.0) - 165.0,
                (WINDOW_HEIGHT / 2.0) - 60.0,
            );

            let subtitle_text = Text::new("Press R to restart", title_font, 30.0);
            let subtitle_text_position = Vec2::new(
                (WINDOW_WIDTH / 2.0) - 170.0,
                (WINDOW_HEIGHT / 2.0) + 10.0,
            );

            let subtitle_text1 = Text::new("Or M to go to Menu", title_font, 20.0);
            let subtitle_text_position1 = Vec2::new(
                (WINDOW_WIDTH / 2.0) - 115.0,
                (WINDOW_HEIGHT / 2.0) + 75.0
            );

            graphics::draw(ctx, &title_text, text_position);
            graphics::draw(ctx, &subtitle_text, subtitle_text_position);
            graphics::draw(ctx, &subtitle_text1, subtitle_text_position1);

            // Game loop drawings
        } else if self.game_started == true && self.game_ended == false {
            let font = Font::from_file_data(ctx, include_bytes!("../resources/RUBBBB__.TTF"));
            let player1_points_text = Text::new(self.player1.points.to_string(), font, 50.0);
            let player1_text_position = Vec2::new(
                (WINDOW_WIDTH / 2.0) - 185.0,
                (WINDOW_HEIGHT / 2.0) - 200.0,
            );

            graphics::draw(ctx, &player1_points_text, player1_text_position);

            let player2_font = Font::from_file_data(ctx, include_bytes!("../resources/RUBBBB__.TTF"));
            let player2_points_text = Text::new(self.player2.points.to_string(), player2_font, 50.0);
            let player2_text_position = Vec2::new(
                (WINDOW_WIDTH / 2.0) + 140.0,
                (WINDOW_HEIGHT / 2.0) - 200.0,
            );

            graphics::draw(ctx, &player2_points_text, player2_text_position);
            

            graphics::draw(ctx, &self.player1.texture, self.player1.position);
            graphics::draw(ctx, &self.player2.texture, self.player2.position);
            graphics::draw(ctx, &self.ball.texture, self.ball.position);

            // Main Menu
        } else if self.game_started == false && self.game_ended == false {
            // let title_font = Font::default();
            let title_font = Font::from_file_data(ctx,include_bytes!("../resources/RUBBBB__.TTF"));
            let title_text = Text::new("* PONG *", title_font, 60.0);
            let text_position = Vec2::new(
                (WINDOW_WIDTH / 2.0) - 140.0,
                (WINDOW_HEIGHT / 2.0) - 60.0,
            );

            let subtitle_text = Text::new("Press SPACE to start", title_font, 30.0);
            let subtitle_text_position = Vec2::new(
                (WINDOW_WIDTH / 2.0) - 185.0,
                (WINDOW_HEIGHT / 2.0) + 10.0,
            );

            let subtitle_text1 = Text::new("Or Q to quit", title_font, 20.0);
            let subtitle_text_position1 = Vec2::new(
                (WINDOW_WIDTH / 2.0) - 70.0,
                (WINDOW_HEIGHT / 2.0) + 70.0,
            );

            graphics::draw(ctx, &title_text, text_position);
            graphics::draw(ctx, &subtitle_text, subtitle_text_position);
            graphics::draw(ctx, &subtitle_text1, subtitle_text_position1);
        }

        Ok(())
    }
}