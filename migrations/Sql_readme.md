### 1. POST /api/{version}/add
操作类型 ：添加记录 SQL语句 ：
```sql
INSERT INTO {sql_table} (file_type, datainfos) VALUES ($1, $2) RETURNING *
```

### POST /api/{version}/check
操作类型 ：查询记录 基本SQL语句 ：
```sql
SELECT * FROM {sql_table} WHERE file_type = $1 AND is_del = $2
```
根据条件变化 ：

- 当提供where_conditions时，会添加相应的条件
- 当audit=true时，会移除is_del条件，显示所有数据
- 当指定fields时，会替换SELECT *为指定字段

### POST /api/{version}/update
操作类型：更新记录 基本SQL语句：
```sql
UPDATE {sql_table} SET datainfos = $1 WHERE file_type = $2 RETURNING *
```
根据条件变化 ：
- 当提供where_conditions时，会添加相应的条件

### POST /api/{version}/isdel
操作类型 ：软删除记录 基本SQL语句 ：
```sql
UPDATE {sql_table} SET {field} = $1 WHERE file_type = $2 RETURNING *
```
说明 ：
- {field}是软删除配置中指定的字段名（如is_del）
- 当提供where_conditions时，会添加相应的条件

### POST /api/{version}/common
操作类型 ：通用CRUD操作 SQL语句 ：根据operation字段值对应上述add/check/update/isdel的SQL语句
- add：对应POST /api/{version}/add
- check：对应POST /api/{version}/check
- update：对应POST /api/{version}/update
- isdel：对应POST /api/{version}/isdel

### POST /api/{version}/common/batch
操作类型 ：批量CRUD操作 SQL语句 ：对每个请求执行上述对应的SQL语句
- 每个请求的operation字段指定具体操作类型
- 其他参数根据对应操作类型的要求提供

### GET /health
操作类型 ：健康检查 SQL语句 ：
```sql
SELECT 1
```

### GET /api/{version}/logs
操作类型：获取日志内容（仅在debug模式下可用） 说明：不执行SQL语句，直接读取日志文件

### 注意事项
1. 所有SQL语句都使用参数化查询，防止SQL注入
2. where_conditions中的字段如果是普通字段（如id、file_type、is_rols、is_del、is_date、created_at、updated_at等），直接使用字段名；如果是JSONB字段，使用 datainfos ->> '字段名' 语法
3. 所有操作都会检查file_type是否在允许的表名列表中
4. 根据service.role配置，只读角色不允许执行写操作（add、update、isdel）
5. {sql_table}表示从配置中获取的数据库表名，通过环境变量SQL_TABLE配置
6. {version}表示API版本号，如v1、v2等
7. 服务配置为单表模式，ALLOWED_TABLES环境变量必须只包含一个表名