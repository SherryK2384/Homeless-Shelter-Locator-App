# Project Details Contract

This Soroban smart contract allows you to store and retrieve the title and description of a project.

## Overview

The `ProjectDetailsContract` provides two main functionalities:

-   **`set_project_details(title: String, description: String)`**: Sets or updates the title and description of the project in the contract's storage.
-   **`get_project_title() -> String`**: Retrieves the currently stored project title. Returns an empty string if no title has been set.
-   **`get_project_description() -> String`**: Retrieves the currently stored project description. Returns an empty string if no description has been set.

## Contract Interface

The contract exposes the following functions that can be called by external users or other contracts:

-   **`set_project_details`**:
    ```
    fn set_project_details(env: Env, title: String, description: String)
    ```
    -   `env`: The Soroban environment.
    -   `title`: The new title of the project as a `String`.
    -   `description`: The new description of the project as a `String`.

-   **`get_project_title`**:
    ```
    fn get_project_title(env: Env) -> String
    ```
    -   `env`: The Soroban environment.
    -   Returns: The current project title as a `String`.

-   **`get_project_description`**:
    ```
    fn get_project_description(env: Env) -> String
    ```
    -   `env`: The Soroban environment.
    -   Returns: The current project description as a `String`.

## Usage

To interact with this contract, you will need a Soroban client library or SDK. Here's a basic example of how you might interact with the contract (this is a conceptual example and the actual implementation might vary depending on the SDK you are using):

1.  **Deploy the contract** to the Soroban network. You will receive a contract ID upon successful deployment.

2.  **Initialize or update project details:**
    ```
    // Assuming you have a client object and the contract ID
    client.call(
        contract_id,
        "set_project_details",
        ["My Awesome Project", "This project aims to do something amazing."]
    );
    ```

3.  **Retrieve project title:**
    ```
    let title = client.call(contract_id, "get_project_title", []);
    // title will contain "My Awesome Project"
    ```

4.  **Retrieve project description:**
    ```
    let description = client.call(contract_id, "get_project_description", []);
    // description will contain "This project aims to do something amazing."
    ```

## Dependencies

-   `soroban_sdk`: The Soroban Software Development Kit.

## Building and Deploying

To build and deploy this contract, you will need to have the Soroban CLI installed and a development environment set up. Please refer to the official Soroban documentation for detailed instructions on building and deploying Soroban contracts.

## Notes

-   The contract uses instance storage to persist the project title and description on the blockchain.
-   The `symbol_short!` macro is used to create compact symbols for the storage keys, which helps in reducing storage costs.
-   The commented-out `log!` statement demonstrates how you could add logging for debugging or auditing purposes. You can uncomment it if needed.

## Contract Details
CC7IIFQWSML47WIUJAX4EC3Q3OR2XVXCURGNKE46BMFCHMRMCHNZVSXZ
![image](https://github.com/user-attachments/assets/9b9b5b89-8db6-4b25-b7a8-a0d88115e4f3)
