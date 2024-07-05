pub mod root {
    use tera::{Context, Tera};
    pub fn render_index_page(tera: &Tera) -> String {
        let mut ctx = Context::new();
        ctx.insert("page", "/");
        tera.render("index.html", &ctx).unwrap()
    }
}

pub mod order {
    use tera::{Context, Tera};
    use crate::model::order::OrderBalance;
    pub fn render_index_page(tera: &Tera, orders: &Vec<OrderBalance<'static>>) -> String {
        let mut ctx = Context::new();
        ctx.insert("page", "/order");
        ctx.insert("orders", orders);
        tera.render("order/index.html", &ctx).unwrap()
    }
    
    pub fn render_order_rows(tera: &Tera, orders: &Vec<OrderBalance<'static>>) -> String {
        let mut ctx = Context::new();
        ctx.insert("orders", orders);
        tera.render("order/rows.html", &ctx).unwrap()
    }
}
