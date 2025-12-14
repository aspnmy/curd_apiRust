### file_type字段与table_name的关系
- file_type字段，表示上传的文件的类型(可以使用的 file_type 有 image 、dicom、avi、mp3、mp4、img2dicom等)
- table_name字段，表示上传的文件的类型对应的表名(如image对应的表名为image_data、dicom对应的表名为dicom_data等)
- 上传文件时，根据file_type字段的值，将文件内容存储到对应的表中
- 查询文件时，根据file_type字段的值，从对应的表中查询文件内容
- image 类型的文件，存储在table_name=image_data表中
- dicom 类型的文件，存储在table_name=dicom_data表中
- avi 类型的文件，存储在table_name=avi_data表中
- mp3 类型的文件，存储在table_name=mp3_data表中
- mp4 类型的文件，存储在table_name=mp4_data表中
- img2dicom 类型的文件，存储在table_name=img2dicom_data表中

