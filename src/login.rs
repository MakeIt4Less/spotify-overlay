pub fn get_login_html() -> String {
    let client_id = env!("client_id");
    let client_secret = env!("client_secret");
    let authorize_info = format!("https://accounts.spotify.com/authorize?response_type=code\
                                &client_id={client_id}\
                                &client_secret={client_secret}\
                                &redirect_uri=http://localhost:6969\
                                &scope=user-modify-playback-state+user-read-currently-playing+user-read-playback-state+user-library-modify+user-library-read");
    format!("<!DOCTYPE html>
            <html lang=\"en\">\
                <head>\
                <meta charset=\"utf-8\">\
                    <title>Hello!</title>\
                </head>\
                <body>\
                    <h1>Hello twitch!</h1>\
                    <p>Hi from Rust</p>\
                    <a href=\"{authorize_info}\">Log in</a>\
                </body>\
            </html>")
}