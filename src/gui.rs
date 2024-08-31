use crate::functions::concate_arrays_to_page;
use crate::MyEguiApp;
use eframe::egui;

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Create a central panel to contain the main UI elements
        egui::CentralPanel::default().show(ctx, |ui| {
            // Set the text style for the UI
            let text_style = egui::TextStyle::Body;
            // Calculate the height of each row based on the text style
            let row_height = ui.text_style_height(&text_style);
            // Get the total number of rows in the menu array
            let total_rows = &self.temp_data.menu_array.len();

            // Create a vertical scrollable area for the main content
            egui::ScrollArea::vertical().min_scrolled_width(450.0).show_rows(ui, row_height, *total_rows, |ui, row_range| {
                // Iterate through each row in the visible range
                for row in row_range {
                    // Get the label for the current row's button
                    let button_label = &self.temp_data.menu_array_parent[row];
                    // Convert the row index to a u8
                    let current_row = row as u8;
                    // Create a unique ID for the collapsing header
                    let open_id = egui::Id::new(&button_label);
                    // Use the row index as the ID source
                    let id_source = &row;
                    // Check if the collapsing header is open
                    let open = ui.data_mut(|data| data.get_temp(open_id)).unwrap_or(false);

                    // Create a collapsing header for each category
                    let toggle: bool = egui::CollapsingHeader::new(button_label.to_string())
                        .id_source(id_source)
                        .open(Some(open))
                        .show(ui, |ui| {
                            // Find the corresponding data for the current category
                            if let Some((associated_string, inner_vec_data)) = self.temp_data.menu_array.iter().find(|(category, _)| *category == button_label) {
                                let accepted_items = &inner_vec_data;
                                // Create buttons for each item in the category
                                for inner_vec in inner_vec_data.iter() {
                                    if ui.button(inner_vec.to_string()).clicked() {
                                        // Set the refresh flag when a button is clicked
                                        self.refresh_output_bool = true;
                                        if self.temp_data.menu_array_pos == current_row && associated_string == button_label && accepted_items.contains(inner_vec) {
                                            // Update selected data and move to next category
                                            self.selected_data_vec.set_string(self.temp_data.menu_array_pos as usize, inner_vec.to_string());
                                            self.temp_data.menu_array_pos = self.temp_data.menu_array_pos + 1;
                                            self.update_toggle_bool = true;
                                        } else {
                                            // Update selected data for the current category
                                            self.selected_data_vec.set_string(row, inner_vec.to_string());
                                        }
                                    };
                                }
                            }
                        })
                        .header_response
                        .clicked();

                    // Handle UI updates based on selection
                    if &current_row == &self.temp_data.menu_array_pos && self.update_toggle_bool {
                        // Toggle the open state of the current category
                        ui.data_mut(|data| data.insert_temp(open_id, !open));
                        if current_row > 0 {
                            // Close the previous category
                            let tmp_button_label = &self.temp_data.menu_array_parent[row - 1];
                            let tmp_open_id = egui::Id::new(&tmp_button_label);
                            ui.data_mut(|data| data.insert_temp(tmp_open_id, open));
                        }
                        self.update_toggle_bool = false;
                    }

                    // Toggle the open state of the collapsing header when clicked
                    if toggle {
                        ui.data_mut(|data| data.insert_temp(open_id, !open));
                    };
                }

                // Refresh the output if the refresh flag is set
                if self.refresh_output_bool {
                    // Initialize vectors to store combined data
                    let mut combined_associated_strings: Vec<&str> = Vec::new();
                    let mut combined_inner_vec_data: Vec<Vec<&str>> = Vec::new();
                    let mut combined_outputs: Vec<Option<&String>> = Vec::new();

                    // Combine data from menu array and selected data
                    for (index, (associated_string, inner_vec_data)) in self.temp_data.menu_array.iter().enumerate() {
                        let output = self.selected_data_vec.get_string(index);
                        combined_associated_strings.push(associated_string);
                        combined_inner_vec_data.push(inner_vec_data.clone());
                        combined_outputs.push(output);
                    }

                    // Print debug information
                    #[cfg(debug_assertions)]
                    {
                        println!("Combined Associated Strings: {:?}", combined_associated_strings);
                        println!("Combined Inner Vec Data: {:?}", combined_inner_vec_data);
                        println!("Combined Outputs: {:?}", combined_outputs);
                    }

                    // Generate the final output string
                    self.temp_data.final_output = concate_arrays_to_page(&combined_associated_strings, &combined_inner_vec_data, &combined_outputs);
                    // Reset the refresh flag
                    self.refresh_output_bool = false;
                }
            });
            // Create a side panel to display the final output
            egui::SidePanel::right("side_panel").exact_width(ctx.available_rect().width() / 2.0).resizable(false).show_animated(ctx, true, |ui| {
                // Create a vertical scroll area for the output
                egui::ScrollArea::vertical().id_source("output_scroll_area").max_height(ui.available_height()).show(ui, |ui| {
                    // Center and justify the output text
                    ui.centered_and_justified(|ui| {
                        // Display the final output as a monospaced label
                        ui.label(egui::RichText::new(&self.temp_data.final_output).monospace()).on_hover_text_at_pointer("Use Ctrl+A to select all then Ctrl+C to copy");
                    });
                });
            })
        });
    }
}
