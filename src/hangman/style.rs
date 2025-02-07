use eframe::egui;

pub fn styles() -> egui::Style {
    let mut style = egui::Style::default(); //calling .default() initializes a Style instance with the default configuration provided by egui
    //background color
    style.visuals.window_fill = egui::Color32::from_rgb(38, 41, 44);
    //text color
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(246, 247, 235)); 
    style
}


pub fn homeScreenPanel() -> egui::Frame {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(53, 96, 90))
        //.stroke(egui::Stroke::new(2.0, egui::Color32::WHITE)) // White border with 2px thickness
        .inner_margin(45.0) // Padding inside the frame
    
}