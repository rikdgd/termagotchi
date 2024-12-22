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
            area: Rect::new(0, 0, 20, 100),
        }
    }
    
    pub fn get_widget(&self) -> impl Widget + 'a {
        let canvas = Canvas::default()
            .block(Block::bordered().title("Stats"))
            .x_bounds([0.0, 20.0])  // TODO: Get based of area
            .y_bounds([0.0, 80.0])
            .paint(|ctx|{
                ctx.draw(&Line {
                    x1: 0.0,
                    y1: 10.0,
                    x2: 10.0,
                    y2: 10.0,
                    color: Color::White,
                });
                
                Self::draw_stats(ctx, self.friend);
            });

        canvas
    }
    
    fn draw_stats(ctx: &mut Context, friend: &Friend) {
        let food_draw = Self::calc_required_symbols(friend.food());
        let joy_draw = Self::calc_required_symbols(friend.joy());
        let energy_draw = Self::calc_required_symbols(friend.energy());
        let waste_draw = Self::calc_required_symbols(friend.waste_level());
        
        for i in 0..food_draw {
            let shape = PixelVectorShape::from_pixel_image(&StatShape::Food)
                .translate(
                    (i * STAT_SHAPE_WIDTH) as i32,
                    0,
                );
            
            ctx.draw(&shape);
        }
        
        todo!()
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