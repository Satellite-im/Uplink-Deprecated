# Use [heroicons](https://heroicons.com/) in [Dioxus](https://dioxuslabs.com/) as Components

This library provides two components, `Icon`, and `IconButton`, which will
generate SVG for a [heroicons](https://heroicons.com/) icon.

```rust
use dioxus::prelude::*;
use dioxus_heroicons::{Icon, IconButton, solid::Shape};

#[inline_props]
fn DeleteButton(cx: Scope, foo: u8) -> Element {
    let onclick = move |evt| {
        // Delete a thing
    };
    let disabled = if foo < 42 { true } else { false} };
    cx.render(rsx! {
        IconButton {
            onclick: onclick,
            class: "some-css-class",
            title: "Delete it",
            disabled: disabled,
            size: 30,
            icon: Shape::Trash,
        }
    })
}

fn PointsRight(cx: Scope) -> Element {
    cx.render(rsx! {
        Icon {
            icon: Shape::ArrowRight,
            fill: "blue",
        }
    })
}
```

See the [library documentation](https://docs.rs/dioxus-heroicons/latest/) for
more details.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
