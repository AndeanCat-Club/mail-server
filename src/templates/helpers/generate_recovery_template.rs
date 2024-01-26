pub fn generate_recovery_html(username: &str, url: &str) -> String {
    let html_template = format!(
        r#"
        <body>
        <div class="main" style="width: 100%; height: 100%;">
            <div style="height: 100px;">
                <img style="height:70px; border-radius: 200px; margin-top: 15px;" src="https://andeancat.club/wp-content/uploads/2023/09/andean-cat-150x150.png"/>
            </div>
            <div class="principal-container" style="width: 95%; margin-left: 2.5%;">
                <div class="body-container" style="padding-top: 50px; border: 1px solid rgba(0, 0, 0, .1); border-left: 0px; border-right: 0px;">
                    <div class="title-container">
                        <p class="thanks-phrase"
                            style="text-align: center; font-family: &quot;Roboto&quot;, sans-serif;outline: 0; font-size: 25px;margin-bottom: 0px;margin-top: 10px;margin-left: 8px;color: #4573ea;">
                            Intrucciones de recuperación<br> de contraseña en Nima!</p>
                        <p class="order-description"
                            style="margin-left: 15px;font-size: 18px; padding-top: 20px; font-family: &quot;Roboto&quot;, sans-serif;">
                            Hola! {}<br><br>Se generado una solicitud para recuperar tu contraseña en tu cuenta de Nima.<br> haz click en el siguiente botón.</p>
                    </div>
    
                    <a href="{}" target="_blank" style="cursor:pointer;"><button class="button" style="font-family: &quot;Roboto&quot;, sans-serif;outline: 0;width: 50%;margin-left: 25%; margin-top: 15px; margin-bottom: 15px; border: 0;padding-top: 15px;padding-bottom: 15px;background-color: #4573ea;color: white;font-size: 17px;font-weight: 700;cursor: pointer;border-radius: 3px;"> Cambia tu contraseña</button></a>
                </div>
                <img style="width: 60%; margin-top: 20px; margin-left: 20%;" src="https://medicapp-cdn.nyc3.cdn.digitaloceanspaces.com/medic-person.png" />
            </div>
        </div>
    </body>
    "#,
        username, url
    );

    html_template
}