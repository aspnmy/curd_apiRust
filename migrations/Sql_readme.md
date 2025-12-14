### 1. POST /api/add
操作类型 ：添加记录 SQL语句 ：
```sql
INSERT INTO common_data (table_name, datainfos) VALUES ($1, $2) RETURNING *
```

### POST /api/check
操作类型 ：查询记录 基本SQL语句 ：
```sql
SELECT * FROM common_data WHERE table_name = $1 AND is_del = $2
```
根据条件变化 ：

- 当提供where_conditions时，会添加相应的条件
- 当audit=true时，会移除is_del条件，显示所有数据
- 当指定fields时，会替换SELECT *为指定字段

### POST /api/update
操作类型：更新记录 基本SQL语句：
```sql
UPDATE common_data SET datainfos = $1 WHERE table_name = $2 RETURNING *
```
根据条件变化 ：
- 当提供where_conditions时，会添加相应的条件

### POST /api/isdel
操作类型 ：软删除记录 基本SQL语句 ：
```sql
UPDATE common_data SET {field} = $1 WHERE table_name = $2 RETURNING *
```
说明 ：
- {field}是软删除配置中指定的字段名（如is_del）
- 当提供where_conditions时，会添加相应的条件

### POST /api/common
操作类型 ：通用CRUD操作 SQL语句 ：根据operation字段值对应上述add/check/update/isdel的SQL语句
- add：对应POST /api/add
- check：对应POST /api/check
- update：对应POST /api/update
- isdel：对应POST /api/isdel

### POST /api/common/batch
操作类型 ：批量CRUD操作 SQL语句 ：对每个请求执行上述对应的SQL语句
- 每个请求的operation字段指定具体操作类型
- 其他参数根据对应操作类型的要求提供

### GET /health
操作类型 ：健康检查 SQL语句 ：
```sql
SELECT 1
```
### GET /api/logs
操作类型：获取日志内容（仅在debug模式下可用） 说明：不执行SQL语句，直接读取日志文件

### 注意事项
1. 所有SQL语句都使用参数化查询，防止SQL注入
2. where_conditions中的字段如果是普通字段（如id、table_name、is_del等），直接使用字段名；如果是JSONB字段，使用 datainfos ->> '字段名' 语法
3. 所有操作都会检查table_name是否在允许的表名列表中
4. 根据service.role配置，只读角色不允许执行写操作（add、update、isdel）