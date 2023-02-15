# Database Migrations

"Migrations" are a database management concept that (in a perfect world) makes updating tables easier. The database keeps track of what migrations have been run in the `__diesel_schema_migration` table.

All pending migrations are applied automatically by the api server when it starts up.

### Runbook

Generate a new migration
```
diesel migration generate NAME
```

Redo the last migration
```
diesel migration redo --database-url postgres://vscode:notsecure@db/survey_app
```

Revert a migration
```
diesel migration revert --database-url postgres://vscode:notsecure@db/survey_app
```