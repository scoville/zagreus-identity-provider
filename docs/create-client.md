## Create Client

Zagreus comes with no default client built for you, and it's up to you to create your own.

In order to do so, only a few things are strictly required:

1. Have a `.env` file at the root of your client project folder
2. Have templates (i.e. html files) _for the routes you need_ located anywhere in your project (typically in a `templates` folder)
3. _Optionally_ have static files (images, css, js, etc...) localed anywhere in your project
4. _Optionally_ make your client a node module with a `package.json` file in order to build a more dyamic client

### `.env`

This file is the "core" of your application as it will orchestrate and configure Zagreus. A simple `.env.` file can look like this:

```env
# Path to the files that should be served statically (css, js, images, etc...)
STATIC_PATH=./static
# Path to the html templates
TEMPLATES_PATH=./templates
# Port used by your client
PORT=5000
# URL where the client is hosted (notice how you can reuse variables)
URL=http://localhost:${PORT}
# The database url that Zagreus must use
DATABASE_URL=postgres://postgres:postgres@localhost/zagreus
# The whitelisted urls (typically your resource api)
ACCESS_TOKEN_AUDIENCE=http://localhost:4000/graphql
# Your client id, must be the same as the one provided to the `zagreus init` command
CLIENT_ID=my_client
# Your client secret that must match the hydra configuration
CLIENT_SECRET=superSecret
# URL to redirect to after registration and login
REDIRECT_URL=http://localhost:4000/callback
# The Hydra admin api url (required by Zagreus)
HYDRA_ADMIN_API_URL=http://localhost:4445
# The Hydra public api url (required by Zagreus)
HYDRA_PUBLIC_API_URL=http://localhost:4444
```

### Templates

Templates are written in html and supports the [Tera](https://tera.netlify.app/) syntax (which is very close to the [Jinja](https://jinja.palletsprojects.com) syntax for Python users). Some variables will be injected in your template as described below.

Notice that not all the templates are required, if you decide not to have a `home` page for instance a `404` is returned when the users visit the `/` path and nothing is done server side.

Also, all features of Tera can be used, including macros or layout.

Here are the available routes (as of today):

- `home`: `/` - _No variables injected_
- `login`: `/login` - `login_challenge`: `string`
- `invitations`: `/invitations` - `invitations`: `{ email: string, path: string }[]`
- `invitation`: `/invitation/:code` - `invitation_challenge`: `string` and `email`: `string`
