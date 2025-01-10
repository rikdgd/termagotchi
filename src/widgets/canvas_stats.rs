use ratatui::{
    layout::Rect,
    widgets::{
        canvas::Canvas,
        Block,
        Widget
    },
};
use ratatui::widgets::canvas::Context;
use crate::friend::Friend;
use crate::shapes::{PixelVectorShape, StatShape};
use crate::utils::Stat;


const STAT_SHAPE_WIDTH: u8 = 10;
const AREA: Rect = Rect::new(0, 0, 30, 100);


pub struct StatsWidget<'a> {
    friend: &'a Friend,
}
impl<'a> StatsWidget<'a> {
    pub fn new(friend: &'a Friend) -> Self {
        Self { 
            friend,
        }
    }
    
    pub fn get_widget(&self) -> impl Widget + 'a {
        Canvas::default()
            .block(Block::bordered().title("Stats"))
            .x_bounds([0.0, f64::from(AREA.width)])
            .y_bounds([0.0, f64::from(AREA.height)])
            .paint(|ctx| Self::render_stats(ctx, self.friend))
    }
    
    fn render_stats(ctx: &mut Context, friend: &Friend) {
        Self::draw_stat(ctx, StatShape::Food, friend.food());
        Self::draw_stat(ctx, StatShape::Joy, friend.joy());
        Self::draw_stat(ctx, StatShape::Sleep, friend.energy());
        Self::draw_stat(ctx, StatShape::Health, friend.health());
    }
    
    fn draw_stat(ctx: &mut Context, shape: StatShape, stat: &Stat) {
        let count = Self::calc_required_symbols(stat);
        
        let draw_row = match shape {
            StatShape::Food => 0,
            StatShape::Joy => 1,
            StatShape::Sleep => 2,
            StatShape::Health => 3,
        };
        
        for i in 0..count {
            let y_move = AREA.height - (draw_row * (STAT_SHAPE_WIDTH + 2)) as u16 - STAT_SHAPE_WIDTH as u16;
                        
            let shape = PixelVectorShape::from_pixel_image(&shape)
                .translate(
                    (i * STAT_SHAPE_WIDTH) as i32,
                    y_move as i32,
                );
            
            ctx.draw(&shape);
        }
    }
    
    fn calc_required_symbols(stat: &Stat) -> u8 {
        match stat.value() {
            0 => 0,
            1..=33 => 1,
            34..=66 => 2,
            _ => 3,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::Stat;
    use crate::widgets::StatsWidget;

    #[test]
    fn calc_required_stat_symbols() {
        let stat_20 = Stat::new(20).unwrap();
        let stat_0 = Stat::new(0).unwrap();
        let stat_100 = Stat::new(100).unwrap();
        let stat_76 = Stat::new(76).unwrap();
        let stat_51 = Stat::new(51).unwrap();
        let stat_26 = Stat::new(26).unwrap();
        let stat_1 = Stat::new(1).unwrap();

        let result_20 = StatsWidget::calc_required_symbols(&stat_20);
        let result_0 = StatsWidget::calc_required_symbols(&stat_0);
        let result_100 = StatsWidget::calc_required_symbols(&stat_100);
        let result_76 = StatsWidget::calc_required_symbols(&stat_76);
        let result_51 = StatsWidget::calc_required_symbols(&stat_51);
        let result_26 = StatsWidget::calc_required_symbols(&stat_26);
        let result_1 = StatsWidget::calc_required_symbols(&stat_1);
        
        assert_eq!(result_20, 1);
        assert_eq!(result_0, 0);
        assert_eq!(result_100, 3);
        assert_eq!(result_76, 3);
        assert_eq!(result_51, 2);
        assert_eq!(result_26, 1);
        assert_eq!(result_1, 1);
    }
}