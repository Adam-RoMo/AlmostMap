# AlmostMap

## Diesel

### Create a migration with Diesel

```bash
cd database
diesel migration generate MIGRATION_NAME
```
### Run the migration

```bash
diesel migration run
```

### Rollback the migration

```bash
diesel migration redo
```