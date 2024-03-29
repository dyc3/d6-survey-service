# D6 project architecture and scope proposal

Project: Test/Survey system: In this project, you need to create a test/survey system. Users are in two roles: creators and takers. The creators can create, save, modify a test/survey. The taker can take a test or a survey.

Regardless of which project we do, we will most likely be doing a lot of front end, UI work.

### Goals

- The UI needs to be quick and responsive.
- Survey Responders should be able to edit their responses.


### Technology Stack and Justification

- Frontend: Typescript using Svelte and Vite (maybe SvelteKit?)
  - Svelte does as much as possible at compile time, which means that the code that runs in the browser is as small and as fast as possible without sacrificing developer experience too much.
  - Has good typescript support.
  - React is not lightweight, and it encourages patterns that are frustrating to work with.
- Backend: Rust using the Rocket framework
  - Request input validation is handled by the framework, automatically, guarenteed at compile time.
    - This will save us tons of time when it comes to writing tests.
  - Relatively mature.
- Persistance: Postgres or MongoDB
  - Compatible with Rocket.

We may be able to easily share type definitions between the frontend and backend using [typeshare](https://github.com/1Password/typeshare), which will save time.

### Code Style, Formatting, and Linting

- Rust: rustfmt + clippy
- Typescript: Prettier

# User Stories

```mermaid
---
title: Use Case Diagram
---
flowchart LR
    UseA([Create/Edit Survey])
    UseB([View Survey Responses])
    UseC([Take Survey])
    UseD([Edit Survey Response])
    UseE([Share Survey])
    Creator[fa:fa-person Creator]
    Responder[fa:fa-person Responder]

    Creator --> UseA
    Creator --> UseB
    Creator --> UseE
    Responder --> UseC
    Responder --> UseD
    UseC -->|requires| UseE
    UseD -->|requires| UseC
```

### Survey Creator

Wants to create a survey and share it with others to gather responses.

```mermaid
---
title: Activity Diagram - Creator
---
flowchart TD
    final((fa:fa-circle))
    start((start))
    style start fill:#000,stroke:transparent,color:#000;
    style final fill:#fff,stroke:#000,color:#000;

    start --> hasAccount{Is user\nregistered?}
    hasAccount -->|Yes| login[Log in]
    hasAccount -->|No| register[Register]
    createSurvey[Create Survey]
    login --> createSurvey
    register --> createSurvey
    addQ[Add/Edit/Remove Question]
    enoughQ{Satisfied w/\nQuestions?}
    saveSurvey[Survey auto-saved]
    createSurvey --> addQ
    addQ --> saveSurvey --> enoughQ
    enoughQ -->|No| addQ
    enoughQ -->|Yes| copyShare[Copy Share Link] -->
    share[Share survey with responders] -->
    wait[Wait for responses] -->
    view[View Results] -->
    final
```

### Survey Responder

Wants to take a survey with as little friction as possible.

```mermaid
---
title: Activity Diagram - Responder
---
flowchart TD
    final((fa:fa-circle))
    start((start))
    style start fill:#000,stroke:transparent,color:#000;
    style final fill:#fff,stroke:#000,color:#000;

    start -->
    recieve[Receive link to survey] --> click[Click link] -->
    addQ[Answer Question]
    enoughQ{Satisfied w/\nAnswers?}
    addQ --> enoughQ
    enoughQ -->|No| addQ
    enoughQ -->|Yes| submit[Submit survey] -->
    final
```

# Architecture

```mermaid
---
title: Service Deployment Structure (server rendered frontend)
---
graph LR
    back[API Service] --> db[(Postgres)]
    back2[API Service] --> db
    back3[API Service] --> db
    front[Frontend] -->|proxy| back
    front2[Frontend] -->|proxy| back2
    front3[Frontend] -->|proxy| back3
    load[Load Balancer] --> front
    load --> front2
    load --> front3
```

In this scenario, the frontend service would take care of rendering any dynamic elements before the page is served.
Each frontend service would be paired with exactly 1 backend service, running on the same machine.

```mermaid
---
title: Entity Relationship Diagram for data storage
---
erDiagram
    User {
        int id PK
        String username
        String password_hash
    }

    Responder {
        UUID uuid
    }

    Survey {
        int id PK
        String title
        String description
        int owner_id FK
        SurveyQuestion[] questions "JSON serialized"
    }

    SurveyQuestion {
        UUID uuid
        bool required
        Question question
    }

    Question {
      String type
      Object content
    }

    SurveyResponse {
        int survey_id FK
        UUID responder_uuid
        Map content "JSON serialized - Question UUID to response content"
    }

    User ||--o{ Survey : owns
    Survey ||--o{ SurveyQuestion : contains
    SurveyQuestion ||--|| Question : contains
    Responder ||--|| SurveyResponse : submits
    Survey ||--o{ SurveyResponse : has
```

```mermaid
---
title: Type relationships
---
classDiagram
    class User {
        +id: i32
        +username: String
        +password_hash: String
        +created_at: NaiveDateTime
        +updated_at: NaiveDateTime
    }

    class NewUser {
        +username: String
        +password_hash: String
    }

    class Survey {
        +id: i32
        +title: String
        +description: String
        +published: bool
        +owner_id: i32
        +questions: SurveyQuestions
    }

    class SurveyPatch {
        +title: Option~String~
        +description: Option~String~
        +published: Option~bool~
        +questions: Option~SurveyQuestions~
    }

    class NewSurvey {
        +owner_id: i32
    }

    class SurveyQuestion {
        +uuid: Uuid
        +required: bool
        +question: Question
    }

    class Question {
        <<Enumeration>>
        Text
        Rating
        MultipleChoice
    }

    class QText {
        +prompt: String
        +description: String
        +multiline: bool
    }

    class QRating {
        +prompt: String
        +description: String
        +max_rating: u8
    }

    class QMultipleChoice {
        +prompt: String
        +description: String
        +multiple: bool
        +choices: Vec~Choice~
    }

    class Choice {
        +uuid: UUID
        +text: String
    }

    User .. NewUser
    SurveyPatch ..> Survey
    Survey .. NewSurvey
    Survey "1" --* "0..*" SurveyQuestion
    SurveyQuestion "1" --* "1" Question
    Question --* QText
    Question --* QRating
    Question --* QMultipleChoice
    QMultipleChoice ..* Choice
    User --o Survey

    class SurveyResponse {
        survey_id: i32
        responder: UUID
        content: HashMap~UUID, Response~
    }

    class Response {
        <<Enumeration>>
        Text
        Rating
        MultipleChoice
    }

    class RText {
        text: String
    }

    class RRating {
        rating: u8
    }

    class RMultipleChoice {
        selected: Vec~UUID~
    }

    SurveyResponse --* Response
    Survey --o SurveyResponse
    SurveyQuestion --> Response
    Response --* RText
    Response --* RRating
    Response --* RMultipleChoice
    RText .. QText
    RRating .. QRating
    RMultipleChoice .. QMultipleChoice
```

- Users can create and own surveys.

## Data Storage

Tables:
- users
- surveys
- responses

When updating survey responses to the database, it will be safe to aquire a [`ROW EXCLUSIVE` lock on the responses table](https://www.postgresql.org/docs/8.1/explicit-locking.html#LOCKING-ROWS) for and updating by using `SELECT FOR UPDATE`.
Questions intentionally don't have their own table, because each question is only ever associated with one survey. They are stored as json in the surveys table. (This is a tradeoff between performance and simplicity. We could have a separate table for questions, but that would require more complex queries to get the questions for a survey.)

## API

The API will be RESTful, and will use JSON for data serialization. As such, it'll be specified using OpenAPI, enforced by [Optic](https://www.useoptic.com/).

Requests will be validated using [Rocket's request guards](https://rocket.rs/v0.5-rc/guide/requests/#request-guards).

Must be capable of the following:
- Status
  - Health check
- Users
  - User registration
  - User login/logout
  - List User's Surveys
- Surveys
  - Create Survey
  - Edit Survey
    - Must not allow editing of published surveys.
    - Must not accept edits of outdated versions of the survey to prevent data loss.
  - Clear all responses to a survey
- Responses
  - Create a new response
  - Edit an existing response

The API is documented in OpenAPI format in [api.yml](api.yml).

### Authentication and Authorization

Users will be authenticated using JWTs.
This means that we don't have to store session data in the database.

If a client makes a request with a bad JWT, the server will respond with a 401 Unauthorized. The client must then enter a logged out state.

```mermaid
sequenceDiagram
    Note over Client: Has invalid/missing JWT, but owns survey 1
    Client->>+Server: PATCH /api/survey/1
    Server->>-Client: 401 Unauthorized
    Note over Client: Client is now logged out
```

If a client makes a request with a good JWT, but is trying to access a resource that they don't have authorization to access, the server will respond with a 403 Forbidden.

```mermaid
sequenceDiagram
    Note over Client: Has valid JWT, but does not own survey 2
    Client->>+Server: PATCH /api/survey/2
    Server->>-Client: 403 Forbidden
    Note over Client: Client remains logged in, shows error
```

### Access Control

Not Authorized: Means the identity of the user is not known.
Forbidden: Means the identity of the user is known, but they don't have permission to access the resource.

```mermaid
---
title: Getting a survey
---

graph TD
    recv[GET /api/survey/1] --> hasjwt{JWT present?}
    hasjwt -->|No| unauth[User is unauthorized]
    hasjwt -->|Yes| auth[User is authorized]
    auth --> isowner{Is owner?}
    unauth --> ispublished{Is published?}
    ispublished -->|Yes| success[Success, return survey]
    ispublished -->|No| forbidden[Forbidden]
    isowner -->|Yes| success[Success, return survey]
    isowner -->|No| forbidden[Forbidden]
```

```mermaid
---
title: Editing a survey
---

graph TD
    recv[PATCH /api/survey/1] --> hasjwt{JWT present?}
    hasjwt -->|No| unauth[User is unauthorized]
    hasjwt -->|Yes| auth[User is authorized]
    auth --> isowner
    isowner{Is owner?}
    ispublished{Is published?}
    forbidden[Forbidden]
    success[Success, Survey updated]
    unauth --> forbidden
    ispublished -->|No| success
    qUpdated{Are Questions being changed?}
    ispublished -->|Yes| qUpdated
    isowner -->|Yes| ispublished
    isowner -->|No| forbidden
    qUpdated -->|Yes| forbidden
    qUpdated -->|No| success
```

### Caching and ETags

The server will support caching using ETags. The client will send an `If-None-Match` header with the ETag of the resource that it has cached. If the server has a resource with the same ETag, it will respond with a 304 Not Modified.

`If-None-Match` and `If-Match` headers should not be required for any requests.

```mermaid
sequenceDiagram
    Client->>+Server: GET /api/survey/{ID}
    Server->>-Client: 200 OK, ETag: foobar
    Client->>+Server: GET /api/survey/{ID}, If-None-Match: foobar
    Server->>-Client: 304 Not Modified
```

Similarly, ETags should be used to prevent data loss when editing a survey, also called a "mid-air collision". The client should send an `If-Match` header with the ETag of the resource that it is editing. If the server has a resource with a different ETag, it will respond with a 409 Conflict.

```mermaid
sequenceDiagram
    actor Alice
    actor Bob
    Alice->>+Server: GET /api/survey/{ID}
    Server->>-Alice: 200 OK, ETag: foo
    Alice->>+Server: PATCH /api/survey/{ID}, If-Match: foo
    Server->>-Alice: 200 OK, ETag: bar
    Bob->>+Server: PATCH /api/survey/{ID}, If-Match: foo
    Server->>-Bob: 409 Conflict
```

This should also be used to make sure survey responders can't submit responses to outdated versions of the survey.

```mermaid
sequenceDiagram
    actor Alice
    actor Bob
    Note over Server: Survey has Etag "foo"
    Alice->>+Server: PATCH /api/survey/{ID}, If-Match: foo
    Server->>-Alice: 200 OK, ETag: bar
    Bob->>+Server: POST /api/survey/{ID}, If-Match: foo
    Server->>-Bob: 409 Conflict
```

#### ETag Generation

ETags should be generated from the JSON representation of the resource, excluding the previous ETag. The ETag should be a MD5 hash of the JSON string.

The ETag should be stored in the database, and should be updated whenever the resource is modified.

# Survey Questions

These are the types of questions that we will support:
- Multiple Choice
  - Eg. "Select one"
  - Eg. "Select all that apply"
- Text (single-line or multi-line)
  - Eg. "What is your name?"
  - Eg. "Describe a time when you..."
- Rating
  - Eg. "On a scale of 1-10, How much do you like this?"

# Response UI Behavior

As a responder completes the survey, the partial response should be saved in the browser's local storage.

When a responder submits the survey, the client must obtain a responder UUID from the server in the response. This UUID will be used to identify the responder in the database, and will allow a responder to edit their response after they have submitted it.

```mermaid
sequenceDiagram
    Client->>+Server: POST /api/survey/{ID}
    Server->>-Client: Success! Here is your Responder UUID
    Client->>+Server: PATCH /api/survey/{ID}?responder={UUID}
    Server->>-Client: Success! Modified response saved.
```

```mermaid
sequenceDiagram
    Client->>+Server: PATCH /api/survey/{ID}
    Server->>-Client: Failed! No responder provided
```

- Closing the broswer tab without submitting will prompt the user before exiting.
- At the end of the survey, the user will be provided with a link that allows them to edit their responses.

# Survey Results

Users must be able to view the responses of the survey. We'll do this by providing the ability to export the responses as a CSV file.

The CSV file will contain the following columns:

- Responder ID
- Tithe of each quesetion, one column per question

The rows will contain the responses of each responder.


