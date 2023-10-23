# GitHub Issue Creator

## Introduction

GitHub Issue Creator is a Rust application that simplifies the process of creating GitHub issues from a `JSON` file and adding them into a GitHub project board. The app is designed to create GitHub issues using the GitHub GraphQL API with data from a `JSON` file.

## Getting Started

This section will guide you through the process of setting up and running the application.

### Prerequisites

Before you can run the application, you'll need to ensure that you have the following:

- A `GH_ACCESS_TOKEN` with necessary permissions such as:
    - `repo`
    - `workflow`
    - `admin:org`
    - `project`
- A GitHub repository where the issues will be created.
- A GitHub project linked to the GitHub repository where the issues will be added.
- A JSON file with the data for the issues you want to create in the same directory as your binary.

### Installation

The installation process involves downloading the binaries from the release assets and running them on your platform. Here's how you can do it:

#### Downloading Binaries from Release Assets

The binaries for the application are automatically built and added to the release assets every time a new release is created. You can download them directly from there.

#### For Ubuntu

1. Open your terminal.
2. Navigate to the directory where you downloaded the binary.
3. Make the binary executable by running the following command:
    ```bash
    chmod +x gh-issue-creator
    ```
4. You can now run the application with the following command:
    ```bash
    ./gh-issue-creator
    ```

#### For Windows

1. Open Command Prompt.
2. Navigate to the directory where you downloaded the binary.
3. You can now run the application with the following command:
    ```bash
    ./gh-issue-creator
    ```

#### For MacOS

1. Open Terminal.
2. Navigate to the directory where you downloaded the binary.
3. You can now run the application with the following command:
    ```bash
    ./gh-issue-creator
    ```

### Configuration

Before you can run the application, you'll need to set up a `.env` file in the same directory as your binary. This file will store important information such as your access token, repository name, and project number.

#### Setting up the .env File

Here's how you can set up your `.env` file:

1. In the same directory as your binary, create a new file and name it `.env`.
2. Open the `.env` file in a text editor.
3. Add the following lines to the file, replacing `<value>` with your actual information:

```bash
ACCESS_TOKEN=<your_access_token>
GH_GRAPHQL_API_URL=https://api.github.com/graphql
REPO_NAME=<your_repository_name>
REPOSITORY_OWNER=<repository_owner_name>
JSON_FILE_PATH=<path_of_the_json_file>
PROJECT_NUMBER=<your_project_number>
```

4. Save and close the file.

Now, your application is configured and ready to run!

## Running the App

Once you've downloaded the binary, set up your `.env` file, and ensured that a JSON file is present in the same directory, you're ready to run the application.

Here's how you can do it:

1. Open your terminal (Command Prompt for Windows users).
2. Navigate to the directory where your binary and `.env` file are located.
3. Run the application with the following command:

    ```bash
    ./gh-issue-creator
    ```

The application will now read the data from your JSON file, create GitHub issues using the GitHub GraphQL API, and add them to the repository project board.

### JSON File Format

The application requires a JSON file in the same directory as your binary. This file should contain the data for the issues you want to create.

#### Sample JSON File

Here's an updated example of what your JSON file should look like:

```json
[
    {
        "title": "Issue 1",
        "description": {
            "body": "This is the body of issue 1",
            "additional_info": "Additional information for issue 1"
        }
    },
    {
        "title": "Issue 2",
        "description": {
            "body": "This is the body of issue 2",
            "additional_info": "Additional information for issue 2"
        }
    }
]
```

Each object in the array represents an issue. The `title` field is the title of the issue, and the `description` field is an object that contains the description of the issue and any additional nested information.

## Contributing

We welcome contributions from everyone. Here's how you can get started:

1. **Fork the Repository**: Start by forking this repository to your own GitHub account.
2. **Clone the Repository**: Next, clone the repository to your local machine so you can start working on it.
3. **Create a New Branch**: Always create a new branch for each feature or fix. This keeps the main branch clean and makes it easier to review and manage contributions.
4. **Make Your Changes**: Now you're ready to make your changes. Whether it's adding a new feature, fixing a bug, or improving documentation, your contributions are always appreciated!
5. **Commit Your Changes**: Once you've made your changes, commit them with a clear and descriptive commit message.
6. **Push Your Changes**: Push your changes to your forked repository on GitHub.
7. **Submit a Pull Request**: Finally, submit a pull request so your changes can be reviewed and merged into the main branch.

Thank you for considering contributing to this project!
