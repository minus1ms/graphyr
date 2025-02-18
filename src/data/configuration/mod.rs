use std::{
    fs::File,
    io::{Read, Write},
    sync::atomic::{AtomicU32, Ordering},
};

use floem::{
    action::{open_file, save_as},
    file::{FileDialogOptions, FileSpec},
    kurbo::Stroke,
    prelude::*,
    taffy::FlexDirection,
};
use layer::Layer;
use serde::{Deserialize, Serialize};

use super::Data;
use crate::utils::signal_serde;

pub mod arrow;
pub mod layer;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(with = "signal_serde")]
    pub show_border: RwSignal<bool>,
    #[serde(with = "signal_serde")]
    pub layers: RwSignal<Vec<Layer>>,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            show_border: RwSignal::new(true),
            layers: RwSignal::new(vec![Layer::new()]),
        }
    }

    pub fn build_view(&self, data: RwSignal<Data>, temp_data: RwSignal<Option<Vec<u8>>>) -> Stack {
        let layers = self.layers;
        let show_border = self.show_border;
        let layer_counter = AtomicU32::new(0);
        v_stack((
            h_stack((
                "Configuration:".style(|s| s.font_bold().font_size(15)),
                button("save").on_click_cont({
                    move |_| {
                        save_as(
                            FileDialogOptions::new()
                                .title("Save Configuration")
                                .default_name("config.ron"),
                            {
                                move |file_info| {
                                    if let Some(file) = file_info {
                                        // serialize data and save to file
                                        // let serialized_data =
                                        //     bincode::serialize(&data.get_untracked()).unwrap();
                                        // let mut file = File::create(&file.path()[0]).unwrap();
                                        // file.write_all(&serialized_data).unwrap();

                                        let serialized_data =
                                            ron::to_string(&data.get_untracked()).unwrap();
                                        let mut file = File::create(&file.path()[0]).unwrap();
                                        file.write_all(&serialized_data.as_bytes()).unwrap();
                                    }
                                }
                            },
                        );
                    }
                }),
                button("load").on_click_cont(move |_| {
                    open_file(
                        FileDialogOptions::new()
                            .title("Load config")
                            .allowed_types(vec![FileSpec {
                                name: "config",
                                extensions: &["ron"],
                            }]),
                        {
                            let temp_data = temp_data.clone();
                            move |file_info| {
                                if let Some(file) = file_info {
                                    let mut file = File::open(&file.path()[0]).unwrap();
                                    let mut buffer = Vec::new();
                                    file.read_to_end(&mut buffer).unwrap();
                                    temp_data.set(Some(buffer));
                                }
                            }
                        },
                    );
                }),
            ))
            .style(|s| s.items_center().gap(5)),
            empty(),
            h_stack((Checkbox::new_rw(show_border.clone()), "Show borders"))
                .style(|s| s.items_center().gap(5)),
            empty(),
            h_stack((
                "Layers:",
                button("+").action(move || layers.update(|layers| layers.push(Layer::new()))),
            ))
            .style(|s| s.items_center().gap(10)),
            dyn_stack(
                move || layers.get().into_iter().enumerate(),
                move |_| layer_counter.fetch_add(1, Ordering::Relaxed),
                move |(i, layer)| {
                    let arrows = layer.arrows.clone();
                    let arrow_counter = AtomicU32::new(0);
                    v_stack((
                        h_stack((
                            Checkbox::new_rw(layer.enabled.clone()),
                            text_input(layer.name),
                            button("x").action(move || {
                                layers.update(|layers| {
                                    layers.remove(i);
                                });
                            }),
                        ))
                        .style(|s| s.items_center().gap(5)),
                        dyn_stack(
                            move || arrows.get().into_iter().enumerate(),
                            move |_| arrow_counter.fetch_add(1, Ordering::Relaxed),
                            move |(i, _)| {
                                h_stack((
                                    button("x").action(move || {
                                        arrows.update(|arrows| {
                                            arrows.remove(i);
                                        });
                                    }),
                                    "arrow",
                                ))
                                .style(|s| s.gap(5).items_center())
                            },
                        )
                        .style(move |s| {
                            if !arrows.get().is_empty() {
                                s.padding(10).gap(10)
                            } else {
                                s
                            }
                            .flex_direction(FlexDirection::Column)
                        })
                        .scroll()
                        .style(move |s| {
                            if !arrows.get().is_empty() {
                                s.border(Stroke::new(1.0))
                            } else {
                                s
                            }
                            .justify_center()
                            .max_height(100)
                        }),
                    ))
                    .style(move |s| {
                        if !arrows.get().is_empty() {
                            s.gap(10)
                        } else {
                            s
                        }
                    })
                },
            )
            .style(|s| {
                s.flex_direction(FlexDirection::Column)
                    .width_full()
                    .padding(10)
                    .gap(10)
            })
            .scroll()
            .style(|s| s.border(Stroke::new(1.0)).padding_right(20)),
            empty(),
        ))
        .style(|s| {
            s.padding(10)
                .items_center()
                .border(Stroke::new(1.0))
                .gap(10)
        })
    }
}
