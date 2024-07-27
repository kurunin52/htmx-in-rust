pub mod root {
    use tera::{Context, Tera};
    pub fn render_index_page() -> String {
        let tera = Tera::new("src/view/templates/*.html").unwrap();

        let mut ctx = Context::new();
        ctx.insert("page", "/");
        tera.render("index.html", &ctx).unwrap()
    }
}

pub mod todo {
    use crate::model::todo::Todo;
    use tera::{Context, Tera};
    pub fn render_index_page(todos: &Vec<Todo>) -> String {
        let tera = Tera::new("src/view/templates/**/*.html").unwrap();

        let mut ctx = Context::new();
        ctx.insert("page", "/todo");
        ctx.insert("todos", todos);
        tera.render("todo/index.html", &ctx).unwrap()
    }
}

pub mod order {
    use crate::model::order::OrderBalance;
    use tera::{Context, Tera};
    pub fn render_index_page(orders: &Vec<OrderBalance>) -> String {
        let tera = Tera::new("src/view/templates/**/*.html").unwrap();

        let mut ctx = Context::new();
        ctx.insert("page", "/order");
        ctx.insert("orders", orders);
        tera.render("order/index.html", &ctx).unwrap()
    }

    pub fn render_order_rows(orders: &Vec<OrderBalance>) -> String {
        let tera = Tera::new("src/view/templates/**/*.html").unwrap();

        let mut ctx = Context::new();
        ctx.insert("orders", orders);
        tera.render("order/rows.html", &ctx).unwrap()
    }
}
