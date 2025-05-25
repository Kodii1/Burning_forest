use plotters::prelude::*;

pub fn create_plot(data: &Vec<(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new("wykres.png", (1600, 1200)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Wykres spalania lasu", ("sans-serif", 40))
        .build_cartesian_2d(0f32..100f32, 0f32..100f32)?;

    ctx.configure_mesh()
        .x_desc("Procent gęstości drzew")
        .y_desc("Procent spalenia drzew")
        .x_labels(11)
        .y_labels(11)
        .x_label_formatter(&|v| format!("{:.0}%", v))
        .y_label_formatter(&|v| format!("{:.0}%", v))
        .x_label_offset(5)
        .y_label_offset(5)
        .draw()?;

    ctx.draw_series(LineSeries::new(data.clone(), &RED))?;

    ctx.draw_series(PointSeries::<_, _, Circle<_, _>, _>::new(
        data.clone(),
        2,
        &RED,
    ))?;

    Ok(())
}
