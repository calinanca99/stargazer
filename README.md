# Stargazer

`stargazer` is a CLI that display the most popular Github repositories of a user. It's based on an assignment I had to do a while ago when I applied for some Rust role.

## Up and running

**DON'T EDIT the `.cache` file manually !!!**

### Get a user's repos

```bash
cargo build --release
./target/release/stargazer -u octocat
```

### Seek help

```bash
./target/release/stargazer --help
```

## The original assignment

You are designing and implementing a CLI tool to help you find out a users most popular repositories using the Github v3 API. In your CLI, build out the following functionality:

1. Add a command that accepts a Github username and displays the top 10 repositories for that user sorted by number of stars. For each repository displayed, show the following fields:

- Repository name
- Repository URL
- Repository description
- Star count

2. Cache these results so that you only make a network request when there is no cached data, or a flag is passed to explicitly clear the cache

3. Allow the user to pass a flag to specify a file format and path that the results will be saved to. The user may decide to save both formats to different paths in the same call. Silence the terminal display in this case. Support the following two formats:

- JSON
- Toml

4. Colour and format the output displayed to the terminal so it is easier on the eyes

- Use a different colour for the field name and field value
- Align the separator between the field name and field value
- Add an option that allows you to pass in an Access Token to increase Github's API limits
