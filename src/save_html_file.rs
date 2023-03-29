use std::error::Error;
use std::fs::File;
use std::io::Write;

pub(crate) fn save_file() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("error.html")?;

    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta http-equiv="X-UA-Compatible" content="ie=edge">
            <title>Strength Level Wrapped</title>
            <style>
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }

                body {
                    font-family: Arial, Helvetica, sans-serif;
                    font-size: 16px;
                    line-height: 1.5;
                    color: #333;
                    background-color: #fff;
                }

                .container {
                    width: 100%;
                    max-width: 600px;
                    margin: 0 auto;
                    padding: 20px;
                }

                h1 {
                    font-size: 24px;
                    margin-bottom: 20px;
                    text-align: center;
                }

                p {
                    margin-bottom: 20px;
                    text-align: justify;
                }

                a {
                    color: #007bff;
                    text-decoration: none;
                }

                @media screen and (max-width: 480px) {
                    h1 {
                        font-size: 20px;
                    }
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Strength Level Monthly Wrap</h1>
                <p>This is a sample email designed to be mobile-friendly. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis euismod mollis tortor at tempus. Donec faucibus justo a consectetur sollicitudin. Nam sit amet lorem eget mi blandit bibendum. Sed volutpat massa at tortor laoreet euismod. Suspendisse potenti. Nullam sagittis auctor velit, non blandit purus laoreet a. Curabitur vitae nisi vel nisl consequat dictum.</p>
                <p>Developed by <a href="https://www.oliverlooney.com/">oliverlooney.com</a>.</p>
            </div>
        </body>
        </html>
    "#;

    file.write_all(html.as_bytes())?;

    Ok(())
}
