### 基于file_type的api端点

- 为服务器后端增加基于file_type的通用api端点
- 结构为 /api/common/{file_type}/{add、check、update、isdel}

- 每个api端点的参数和返回值都与common_data表中的datainfos字段相关
- 前端在上传文件时，需要指定file_type字段，后端根据file_type字段的值，将文件存储到对应的表中
- 前端在查询文件时，需要指定file_type字段，后端根据file_type字段的值，从对应的表中查询文件内容
- 前端在更新文件时，需要指定file_type字段，后端根据file_type字段的值，更新对应的表中的记录
- 前端在删除文件时，需要指定file_type字段，后端根据file_type字段的值，将对应的表中的记录软删除

- 通用查询方案，前端在查询文件时，指定file_type字段，后端根据请求中的file_type值，从对应的表中查询文件内容
- 这样前端新建file_type值后，就能自动适配后端的业务，不需要后端再重新编写对应的api端点