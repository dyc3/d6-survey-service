## Setup

1. Install docker and docker-compose
2. Install vscode
3. Clone the repo
4. Open the repo in vscode
5. From the command pallette (<kbd>Ctrl + Shift + P</kbd>) , run `Dev Containers: Reopen in Container`
6. Set up github cli: `gh auth login`
7. Run `./scripts/install-extras.sh`
8. Run `yarn install` to install the frontend dependencies

## Running

Open the terminal in vscode with <kbd>Ctrl + ~</kbd>.

Run `yarn run dev` to start the frontend.
Run `cargo watch -x run` to start the backend.

Both the frontend and backend will automatically reload/recompile when you make changes.

## Testing

TODO

## Creating a Pull Request

Before writing any code or making any commits, make a new branch

```bash
git checkout -b BRANCH_NAME
```

You can now make changes to on this branch. When you are done:

1. Run `gh pr create`
	- Add `--draft` flag to mark it as a work-in-progress draft PR
2. Follow the prompts to submit your pull request.
