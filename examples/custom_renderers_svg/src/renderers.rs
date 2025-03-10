use crate::Form;
use leptos::prelude::*;
use leptos::web_sys;
use leptos_struct_table::*;

const ROW_HEIGHT: usize = 30;
const ROW_HEIGHT_HALF: usize = ROW_HEIGHT / 2;

wrapper_render_fn!(
    /// g
    GRenderer,
    g,
);

#[allow(non_snake_case)]
pub fn SvgTbodyRenderer(
    content: impl IntoView,
    class: Signal<String>,
    body_ref: BodyRef,
) -> impl IntoView {
    view! { <g class=class use:body_ref>{content}</g> }
}

#[allow(unused_variables, non_snake_case)]
pub fn SvgRowRenderer(
    class: Signal<String>,
    row: RwSignal<Form>,
    index: usize,
    selected: Signal<bool>,
    on_select: EventHandler<web_sys::MouseEvent>,
) -> impl IntoView {
    let transform = y_transform_from_index(index);

    view! {
        <g
            class=class
            transform=transform
            on:click=move |mouse_event| on_select.run(mouse_event)
        >
            <line
                x1="5"
                y1="0"
                x2="150"
                y2="0"
                stroke-width="1px"
                stroke="black"
                opacity="0.1"
            ></line>
            {TableRow::render_row(row, index)}
        </g>
    }
}

fn y_transform_from_index(index: usize) -> String {
    format!("translate(0, {})", (index + 1) * ROW_HEIGHT)
}

#[allow(non_snake_case)]
pub fn SvgErrorRowRenderer(err: String, index: usize, _col_count: usize) -> impl IntoView {
    let transform = y_transform_from_index(index);

    view! {
        <g transform=transform>
            <text x="0" y=ROW_HEIGHT_HALF dominant-baseline="central">
                {err}
            </text>
        </g>
    }
}

#[allow(non_snake_case, unstable_name_collisions)]
pub fn SvgLoadingRowRenderer(
    class: Signal<String>,
    _get_cell_class: Callback<(usize,), String>,
    get_inner_cell_class: Callback<(usize,), String>,
    index: usize,
    _col_count: usize,
) -> impl IntoView {
    let transform = y_transform_from_index(index);

    view! {
        <g class=class transform=transform>
            <text x="0" y=ROW_HEIGHT_HALF class=get_inner_cell_class.run((0,)) dominant-baseline="central">
                Loading...
            </text>
        </g>
    }
}

#[component]
pub fn SvgHeadCellRenderer<F>(
    /// The class attribute for the head element. Generated by the classes provider.
    #[prop(into)]
    class: Signal<String>,
    /// The class attribute for the inner element. Generated by the classes provider.
    #[prop(into)]
    inner_class: String,
    /// The index of the column. Starts at 0 for the first column. The order of the columns is the same as the order of the fields in the struct.
    index: usize,
    /// The sort priority of the column. `None` if the column is not sorted. `0` means the column is the primary sort column.
    #[prop(into)]
    sort_priority: Signal<Option<usize>>,
    /// The sort direction of the column. See [`ColumnSort`].
    #[prop(into)]
    sort_direction: Signal<ColumnSort>,
    /// The event handler for the click event. Has to be called with [`TableHeadEvent`].
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(TableHeadEvent) + 'static,
{
    let style = default_th_sorting_style(sort_priority, sort_direction);

    let transform = transform_from_index(index, 0);

    view! {
        <g
            class=class
            transform=transform
            on:click=move |mouse_event| on_click(TableHeadEvent {
                index,
                mouse_event,
            })

            style=style
        >
            <text x="0" y=ROW_HEIGHT_HALF class=inner_class dominant-baseline="central">
                {children()}
            </text>
        </g>
    }
}

#[component]
#[allow(unused_variables)]
pub fn SvgTextCellRenderer<T>(
    class: String,
    value: Signal<T>,
    row: RwSignal<Form>,
    index: usize,
) -> impl IntoView
where
    T: IntoView + Clone + Send + Sync + 'static,
{
    let x = x_from_index(index);

    view! {
        <text x=x y=ROW_HEIGHT_HALF class=class dominant-baseline="central">
            {value}
        </text>
    }
}

#[component]
#[allow(unused_variables)]
pub fn SvgPathCellRenderer(
    #[prop(into)] class: String,
    value: Signal<String>,
    row: RwSignal<Form>,
    index: usize,
) -> impl IntoView {
    let transform = transform_from_index(index, 3);

    view! { <path transform=transform class=class d=value></path> }
}

fn transform_from_index(index: usize, y: usize) -> String {
    format!("translate({}, {y})", x_from_index(index))
}

fn x_from_index(index: usize) -> usize {
    5 + index * 100
}
