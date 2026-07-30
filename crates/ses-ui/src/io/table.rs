//! Sortable data table.

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct TableColumn {
    pub id: String,
    pub title: String,
    pub sortable: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TableRow {
    pub id: String,
    pub cells: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TableDef {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortDir {
    Asc,
    Desc,
}

#[component]
pub fn DataTable(def: TableDef) -> Element {
    let mut sort_col = use_signal(|| None::<usize>);
    let mut sort_dir = use_signal(|| SortDir::Asc);

    let columns = def.columns.clone();
    let mut rows = def.rows.clone();
    if let Some(col_idx) = *sort_col.read() {
        let dir = *sort_dir.read();
        rows.sort_by(|a, b| {
            let av = a.cells.get(col_idx).map(String::as_str).unwrap_or("");
            let bv = b.cells.get(col_idx).map(String::as_str).unwrap_or("");
            let ord = av.cmp(bv);
            match dir {
                SortDir::Asc => ord,
                SortDir::Desc => ord.reverse(),
            }
        });
    }

    rsx! {
        table { class: "ses-data-table",
            thead {
                tr {
                    for (idx, col) in columns.into_iter().enumerate() {
                        {
                            let sortable = col.sortable;
                            let title = col.title.clone();
                            rsx! {
                                th {
                                    key: "{col.id}",
                                    onclick: move |_| {
                                        if !sortable {
                                            return;
                                        }
                                        if sort_col() == Some(idx) {
                                            sort_dir.set(match sort_dir() {
                                                SortDir::Asc => SortDir::Desc,
                                                SortDir::Desc => SortDir::Asc,
                                            });
                                        } else {
                                            sort_col.set(Some(idx));
                                            sort_dir.set(SortDir::Asc);
                                        }
                                    },
                                    "{title}"
                                    if sortable && sort_col() == Some(idx) {
                                        span {
                                            " "
                                            if sort_dir() == SortDir::Asc { "▲" } else { "▼" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            tbody {
                for row in rows {
                    tr { key: "{row.id}",
                        for cell in row.cells.iter() {
                            td { "{cell}" }
                        }
                    }
                }
            }
        }
    }
}
