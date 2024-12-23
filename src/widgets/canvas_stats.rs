use ratatui::{
    style::Color,
    layout::Rect,
    widgets::{
        canvas::{Canvas, Line},
        Block,
        Widget
    },
};
use ratatui::widgets::canvas::Context;
use crate::friend::Friend;
use crate::shapes::{PixelVectorShape, StatShape};
use crate::utils::Stat;


const STAT_SHAPE_WIDTH: u8 = 10;


pub struct StatsWidget<'a> {
    friend: &'a Friend,
    area: Rect,
}
impl<'a> StatsWidget<'a> {
    pub fn new(friend: &'a Friend) -> Self {
        Self { 
            friend,
            area: Rect::new(0, 0, 30, 100),
        }
    }
    
    pub fn get_widget(&self) -> impl Widget + 'a {
        Canvas::default()
            .block(Block::bordered().title("Stats"))
            .x_bounds([0.0, f64::from(self.area.width)])
            .y_bounds([0.0, f64::from(self.area.height)])
            .paint(|ctx| Self::render_stats(ctx, self.friend))
    }
    
    fn render_stats(ctx: &mut Context, friend: &Friend) {
        Self::draw_stat(ctx, StatShape::Food, friend.food());
        Self::draw_stat(ctx, StatShape::Joy, friend.joy());
        Self::draw_stat(ctx, StatShape::Sleep, friend.energy());
        Self::draw_stat(ctx, StatShape::Waste, friend.waste_level());
    }
    
    fn draw_stat(ctx: &mut Context, shape: StatShape, stat: &Stat) {
        let count = Self::calc_required_symbols(stat);
        
        let draw_row = match shape {
            StatShape::Food => 0,
            StatShape::Joy => 1,
            StatShape::Sleep => 2,
            StatShape::Waste => 3,
        };
        
        for i in 0..count {
            let shape = PixelVectorShape::from_pixel_image(&shape)
                .translate(
                    (i * STAT_SHAPE_WIDTH) as i32,
                    (draw_row * STAT_SHAPE_WIDTH) as i32,
                );
            
            ctx.draw(&shape);
        }
    }
    
    fn calc_required_symbols(stat: &Stat) -> u8 {
        match stat.value() {
            0 => 0,
            1..=25 => 1,
            26..=50 => 2,
            51..=75 => 3,
            _ => 4,
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
        assert_eq!(result_100, 4);
        assert_eq!(result_76, 4);
        assert_eq!(result_51, 3);
        assert_eq!(result_26, 2);
        assert_eq!(result_1, 1);
    }
}