mod dom;
mod html;

fn main() {
    let source = r#"
    <html lang="ja">
      <head>
        <title>test</title>
      </head>
      <body>
        <h1>parsing test</h1>
        <p class="greeting" style="color: red">Hello world!</p>

        sample
      </body>
    </html>
"#;

    let dom_tree = html::parse(source.to_string());
    dom::walk_dom_tree(&dom_tree, 0);
}
