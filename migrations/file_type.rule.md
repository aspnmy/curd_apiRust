### file_type字段与table_name的关系
- file_type字段，表示上传的文件的类型(可以使用的 file_type 有 image 、dicom、avi、mp3、mp4、img2dicom等)
- table_name字段，表示上传的文件的类型对应的表名(如image对应的表名为image_data、dicom对应的表名为dicom_data等)
- 上传文件时，根据file_type字段的值，将文件内容存储到对应的表中
- 查询文件时，根据file_type字段的值，从对应的表中查询文件内容
- image 类型的文件，存储在image_data表中
- dicom 类型的文件，存储在dicom_data表中
- avi 类型的文件，存储在avi_data表中
- mp3 类型的文件，存储在mp3_data表中
- mp4 类型的文件，存储在mp4_data表中
- img2dicom 类型的文件，存储在img2dicom_data表中

#### img2dicom 类型的文件特别说明
- img2dicom 类型的文件，是指将image文件如jpg、png、gif、bmp等图片文件转换为dicom文件
- 转换后的dicom文件，包含了image文件的所有信息，如像素数据、元数据等
- 查询img2dicom 类型的文件时，返回的是dicom文件的内容，而不是image文件的内容
- 上传img2dicom 类型的文件时，需要提供image文件的路径，而不是dicom文件的路径
- 查询img2dicom 类型的文件时，返回的是dicom文件的内容，而不是image文件的内容
- img2dicom 表中的datainfos字段中,增加了一个字段image_content,表示上传的image文件的base64编码后的内容，增加一个dicom_path字段,表示转换后的dicom文件的路径，增加一个字段dicom_content,表示dicom文件base64编码后的内容
- 查询img2dicom 类型的文件时，返回的datainfos字段中，包含了dicom_path、dicom_content这2个字段
- 前端渲染的时候如果dicom_content字段中的数据可以直接渲染成功就不需要读取dicom_path字段中的数据，如果直接渲染失败，再读取dicom_path字段中的数据，如果dicom_path字段中的数据渲染失败，则取image_content字段中的数据渲染；
- 前端要自行实现dicom数据的展示，如使用dicom.js库等