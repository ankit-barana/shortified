use rocket::response::content;
// 2px solid rgb(255, 81, 0) - my orange if want to use it
pub const FRONT_END: content::RawHtml<& 'static str> = content::RawHtml(r#"
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
            color: #EEEBDD;
            background-color: #1B1717;
            text-align: center;
        }
        
        body {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            margin-bottom: 60px;
            color: #eeebdd;
            background-color: #1b1717;
            fontFamily: {
                'serif': ['Vollkorn', 'Georgia', 'Tahoma', 'serif'],
                'sans': ['Inter', 'Helvetica', 'Arial', 'sans-serif']
              },
        }
        .tagline {
            color: #DFDCCB;
            font-size: 15px;
            font-weight: normal;
        }

        .logo:not(.tagline) {
            font-weight: bold;
            font-size: 1.7em;
            margin-bottom: 20px;
        }

        .container {
            position-relative;
            display: flex;
            align-items: center;
            padding: 20px;
        }

        #myInput {
            text-align: left;
            font-family: 'Tahoma', serif;
            padding-right: 40px;
            box-shadow: 0.2px 1px 5px #120f0f;
            border: 0.2px solid #120f0f; 
            border-radius: 4px;
            color: #DFDCCB;
            background-color: #2e2727;
            padding: 15px;
            height: 40px;
            width: 45vw;
        }

        #myButton {
            position: absolute;
            right: 330px;
            top: 345px
        }

        .shortURL {
            margin-top: 20px;
        }

    </style>
        <body>
            <div class="logo">
                <h1 >Shortified!</h1>
                <p class="tagline">Shorten 🔗s quickly and effortlessly. Simplify. Share. Go. 🚀</p>   
            </div>
            <div class="container">
                <form action="/" method="post">
                <input type="text" id="myInput" name="url" placeholder="https://www.example.com/demo/" required />
                <button id="myButton" type="submit">Shorten</button>
            </form> 
    </div>
        </body>
    "#);