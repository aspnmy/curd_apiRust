// 从配置文件中获取API基础URL
const API_BASE_URL = UI_CONFIG.API_BASE_URL;

// 当前选中的图片ID
let currentImageId = null;

// 初始化页面
window.addEventListener('DOMContentLoaded', function() {
    // 监听文件选择事件，显示预览
    const imageInput = document.getElementById('imageInput');
    imageInput.addEventListener('change', function(e) {
        const file = e.target.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = function(e) {
                const preview = document.getElementById('imagePreview');
                const img = document.createElement('img');
                img.src = e.target.result;
                preview.innerHTML = '';
                preview.appendChild(img);
                preview.style.display = 'block';
            };
            reader.readAsDataURL(file);
        }
    });
    
    // 加载图片列表
    loadImages();
    
    // 监听搜索框回车事件
    const searchInput = document.getElementById('searchInput');
    searchInput.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            searchImages();
        }
    });
});

// 显示消息
function showMessage(message, type) {
    // 创建消息元素
    const messageDiv = document.createElement('div');
    messageDiv.className = `message ${type}`;
    messageDiv.textContent = message;
    
    // 添加到容器顶部
    const container = document.querySelector('.container');
    container.insertBefore(messageDiv, container.firstChild);
    
    // 3秒后自动移除
    setTimeout(function() {
        if (messageDiv.parentNode) {
            messageDiv.parentNode.removeChild(messageDiv);
        }
    }, 3000);
}

// 上传图片
async function uploadImage() {
    const imageInput = document.getElementById('imageInput');
    const file = imageInput.files[0];
    
    if (!file) {
        showMessage('请先选择一张图片', 'error');
        return;
    }
    
    try {
        // 显示加载状态
        showMessage('正在上传图片...', 'info');
        
        // 将图片转换为Base64
        const base64Image = await fileToBase64(file);
        
        // 构造请求数据（符合服务器期望的格式）
        const requestData = {
            table_name: 'resources',
            operation: 'add',
            data: {
                file_name: file.name,
                file_type: file.type,
                content: base64Image,
                description: `上传的图片: ${file.name}`
            }
        };
        
        // 发送请求
        const response = await fetch(`${API_BASE_URL}/add`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP错误! 状态: ${response.status}`);
        }
        
        const result = await response.json();
        
        if (result.success) {
            showMessage('图片上传成功!', 'success');
            // 清空输入和预览
            imageInput.value = '';
            document.getElementById('imagePreview').style.display = 'none';
            // 刷新图片列表
            loadImages();
        } else {
            throw new Error(result.message || '上传失败');
        }
    } catch (error) {
        showMessage(`上传失败: ${error.message}`, 'error');
        console.error('上传图片错误:', error);
    }
}

// 将文件转换为Base64
function fileToBase64(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = function(e) {
            resolve(e.target.result);
        };
        reader.onerror = function(e) {
            reject(new Error('文件读取失败'));
        };
        reader.readAsDataURL(file);
    });
}

// 加载图片列表
async function loadImages() {
    try {
        // 显示加载状态
        const imageList = document.getElementById('imageList');
        imageList.innerHTML = '<div style="text-align: center; padding: 20px;"><div class="loading"></div> 正在加载图片...</div>';
        
        // 构造请求数据（符合服务器期望的格式）
        const showAll = document.getElementById('showAllCheckbox').checked;
        const requestData = {
            table_name: 'resources',  // 修正字段名
            operation: 'check',       // 修正字段名
            data: {},                 // 重要：data字段是必填的，不能为空
            where_conditions: showAll ? null : [  // 修正格式，showAll为true时设置为null
                { 
                    field: 'is_del', 
                    operator: '=', 
                    value: false 
                }
            ],
            audit: false
        };
        
        // 发送请求
        const response = await fetch(`${API_BASE_URL}/check`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP错误! 状态: ${response.status}`);
        }
        
        const result = await response.json();
        
        if (result.success) {
            renderImageList(result.data || []);
        } else {
            throw new Error(result.message || '加载失败');
        }
    } catch (error) {
        showMessage(`加载图片列表失败: ${error.message}`, 'error');
        console.error('加载图片列表错误:', error);
        document.getElementById('imageList').innerHTML = `<div style="text-align: center; color: red; padding: 20px;">加载失败: ${error.message}</div>`;
    }
}

// 搜索图片
async function searchImages() {
    try {
        const searchInput = document.getElementById('searchInput');
        const keyword = searchInput.value.trim();
        
        if (!keyword) {
            loadImages();
            return;
        }
        
        // 显示加载状态
        const imageList = document.getElementById('imageList');
        imageList.innerHTML = '<div style="text-align: center; padding: 20px;"><div class="loading"></div> 正在搜索图片...</div>';
        
        // 构造请求数据（符合服务器期望的格式）
        const showAll = document.getElementById('showAllCheckbox').checked;
        
        // 构建条件数组
        let where_conditions = [];
        
        // 添加删除状态条件
        if (!showAll) {
            where_conditions.push({
                field: 'is_del',
                operator: '=',
                value: false
            });
        }
        
        // 添加搜索条件
        where_conditions.push({
            field: 'file_name',
            operator: 'LIKE',
            value: `%${keyword}%`
        });
        
        const requestData = {
            table_name: 'resources',
            operation: 'check',
            data: {},                 // 重要：data字段是必填的，不能为空
            where_conditions: where_conditions.length > 0 ? where_conditions : null,
            audit: false
        };
        
        // 发送请求
        const response = await fetch(`${API_BASE_URL}/check`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP错误! 状态: ${response.status}`);
        }
        
        const result = await response.json();
        
        if (result.success) {
            renderImageList(result.data || []);
        } else {
            throw new Error(result.message || '搜索失败');
        }
    } catch (error) {
        showMessage(`搜索图片失败: ${error.message}`, 'error');
        console.error('搜索图片错误:', error);
        document.getElementById('imageList').innerHTML = `<div style="text-align: center; color: red; padding: 20px;">搜索失败: ${error.message}</div>`;
    }
}

// 渲染图片列表
function renderImageList(images) {
    const imageList = document.getElementById('imageList');
    
    if (images.length === 0) {
        imageList.innerHTML = '<div style="text-align: center; color: #666; padding: 20px;">没有找到图片</div>';
        return;
    }
    
    const html = images.map(image => {
        const isDeleted = image.deleted || false;
        return `
            <div class="image-item" data-id="${image.id}">
                <div class="image-item-info">
                    <div>名称: ${image.file_name || '未知'}</div>
                    <div>类型: ${image.file_type || 'unknown'}</div>
                    <div>大小: ${formatFileSize(image.file_size || 0)}</div>
                    <div>ID: ${image.id}</div>
                    ${isDeleted ? '<div style="color: red;">已删除</div>' : ''}
                </div>
                <div style="cursor: pointer;" onclick="showImageDetail(${image.id})">
                    ${image.content ? `<img src="${image.content}" alt="${image.file_name}" onclick="event.stopPropagation()">` : '<div style="height: 150px; background-color: #eee; display: flex; align-items: center; justify-content: center; color: #999;">无预览</div>'}
                </div>
                <div class="image-item-actions">
                    <button onclick="event.stopPropagation(); showImageDetail(${image.id})">详情</button>
                    ${!isDeleted ? `<button onclick="event.stopPropagation(); editImage(${image.id})">修改</button>` : '<button disabled>修改</button>'}
                    <button onclick="event.stopPropagation(); deleteImage(${image.id}, ${isDeleted})">${isDeleted ? '真实删除' : '标记删除'}</button>
                </div>
            </div>
        `;
    }).join('');
    
    imageList.innerHTML = html;
}

// 格式化文件大小
function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// 显示图片详情
async function showImageDetail(id) {
    try {
        // 验证id是否有效
        if (!id || isNaN(id)) {
            showMessage('无效的图片ID', 'error');
            return;
        }
        
        currentImageId = id;
        
        // 显示加载状态
        const detailSection = document.getElementById('detailSection');
        const imageDetail = document.getElementById('imageDetail');
        imageDetail.innerHTML = '<div style="text-align: center; padding: 20px;"><div class="loading"></div> 正在加载详情...</div>';
        detailSection.style.display = 'block';
        
        // 构造请求数据（符合服务器期望的格式）
        const requestData = {
            table_name: 'resources',
            operation: 'check',
            data: {},                 // 重要：data字段是必填的，不能为空
            where_conditions: [
                {
                    field: 'id',
                    operator: '=',
                    value: id
                }
            ],
            audit: true
        };
        
        // 发送请求
        const response = await fetch(`${API_BASE_URL}/check`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP错误! 状态: ${response.status}`);
        }
        
        const result = await response.json();
        
        if (result.success && result.data && result.data.length > 0) {
            const image = result.data[0];
            renderImageDetail(image);
        } else {
            throw new Error(result.message || '未找到图片详情');
        }
    } catch (error) {
        showMessage(`加载图片详情失败: ${error.message}`, 'error');
        console.error('加载图片详情错误:', error);
    }
}

// 渲染图片详情
function renderImageDetail(image) {
    const imageDetail = document.getElementById('imageDetail');
    const isDeleted = image.deleted || false;
    
    const html = `
        <div class="detail-image">
            ${image.content ? `<img src="${image.content}" alt="${image.file_name}">` : '<div style="height: 300px; background-color: #eee; display: flex; align-items: center; justify-content: center; color: #999;">无预览</div>'}
        </div>
        <div class="detail-info">
            <strong>ID:</strong>
            <span>${image.id}</span>
            
            <strong>名称:</strong>
            <span>${image.file_name || '未知'}</span>
            
            <strong>类型:</strong>
            <span>${image.file_type || 'unknown'}</span>
            
            <strong>大小:</strong>
            <span>${formatFileSize(image.file_size || 0)}</span>
            
            <strong>描述:</strong>
            <span>${image.description || '无描述'}</span>
            
            <strong>创建时间:</strong>
            <span>${formatDateTime(image.created_at)}</span>
            
            <strong>更新时间:</strong>
            <span>${formatDateTime(image.updated_at)}</span>
            
            <strong>状态:</strong>
            <span style="color: ${isDeleted ? 'red' : 'green'}">${isDeleted ? '已删除' : '正常'}</span>
            
            ${image.deleted_at ? `
                <strong>删除时间:</strong>
                <span>${formatDateTime(image.deleted_at)}</span>
            ` : ''}
        </div>
        <div style="display: flex; gap: 10px; justify-content: flex-start;">
            ${!isDeleted ? `<button onclick="editImage(${image.id})">修改图片</button>` : ''}
            <button onclick="hideImageDetail()">关闭详情</button>
        </div>
    `;
    
    imageDetail.innerHTML = html;
}

// 隐藏图片详情
function hideImageDetail() {
    document.getElementById('detailSection').style.display = 'none';
    currentImageId = null;
}

// 编辑图片
async function editImage(id) {
    try {
        // 验证id是否有效
        if (!id || isNaN(id)) {
            showMessage('无效的图片ID', 'error');
            return;
        }
        
        currentImageId = id;
        
        // 显示加载状态
        const editSection = document.getElementById('editSection');
        const editForm = document.getElementById('editForm');
        editForm.innerHTML = '<div style="text-align: center; padding: 20px;"><div class="loading"></div> 正在加载编辑表单...</div>';
        editSection.style.display = 'block';
        
        // 获取图片详情（符合服务器期望的格式）
        const requestData = {
            table_name: 'resources',
            operation: 'check',
            data: {},                 // 重要：data字段是必填的，不能为空
            where_conditions: [
                {
                    field: 'id',
                    operator: '=',
                    value: id
                }
            ],
            audit: true
        };
        
        const response = await fetch(`${API_BASE_URL}/check`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP错误! 状态: ${response.status}`);
        }
        
        const result = await response.json();
        
        if (result.success && result.data && result.data.length > 0) {
            const image = result.data[0];
            renderEditForm(image);
        } else {
            throw new Error(result.message || '未找到图片详情');
        }
    } catch (error) {
        showMessage(`加载编辑表单失败: ${error.message}`, 'error');
        console.error('加载编辑表单错误:', error);
    }
}

// 渲染编辑表单
function renderEditForm(image) {
    const editForm = document.getElementById('editForm');
    
    const html = `
        <div class="form-group">
            <label for="editFileName">文件名称:</label>
            <input type="text" id="editFileName" value="${image.file_name || ''}" placeholder="请输入文件名称">
        </div>
        
        <div class="form-group">
            <label for="editFileType">文件类型:</label>
            <input type="text" id="editFileType" value="${image.file_type || ''}" placeholder="请输入文件类型">
        </div>
        
        <div class="form-group">
            <label for="editDescription">描述:</label>
            <textarea id="editDescription" placeholder="请输入描述">${image.description || ''}</textarea>
        </div>
        
        <div class="form-group">
            <label for="editContent">图片内容:</label>
            <input type="file" id="editContent" accept="image/*">
            <div style="margin-top: 10px; font-size: 12px; color: #666;">提示: 如不选择新图片，将保留原有图片内容</div>
        </div>
        
        <div class="edit-actions">
            <button onclick="saveImageChanges()">保存修改</button>
            <button onclick="cancelEdit()">取消编辑</button>
        </div>
    `;
    
    editForm.innerHTML = html;
}

// 保存图片修改
async function saveImageChanges() {
    if (!currentImageId) {
        showMessage('未选择要修改的图片', 'error');
        return;
    }
    
    try {
        const editFileName = document.getElementById('editFileName').value.trim();
        const editFileType = document.getElementById('editFileType').value.trim();
        const editDescription = document.getElementById('editDescription').value.trim();
        const editContent = document.getElementById('editContent').files[0];
        
        // 验证必填项
        if (!editFileName) {
            showMessage('文件名称不能为空', 'error');
            return;
        }
        
        // 显示加载状态
        showMessage('正在保存修改...', 'info');
        
        // 构造请求数据（符合服务器期望的格式）
        let content = null;
        if (editContent) {
            content = await fileToBase64(editContent);
        }
        
        // 构建更新数据
        const update_data = {
            file_name: editFileName,
            file_type: editFileType || undefined,
            description: editDescription || undefined,
            content: content || undefined,
            updated_at: new Date().toISOString()
        };
        
        const requestData = {
            table_name: 'resources',
            operation: 'update',
            where_conditions: [
                {
                    field: 'id',
                    operator: '=',
                    value: currentImageId
                }
            ],
            data: update_data
        };
        
        // 发送请求
        const response = await fetch(`${API_BASE_URL}/update`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP错误! 状态: ${response.status}`);
        }
        
        const result = await response.json();
        
        if (result.success) {
            showMessage('图片修改成功!', 'success');
            cancelEdit();
            hideImageDetail();
            loadImages();
        } else {
            throw new Error(result.message || '修改失败');
        }
    } catch (error) {
        showMessage(`修改图片失败: ${error.message}`, 'error');
        console.error('修改图片错误:', error);
    }
}

// 取消编辑
function cancelEdit() {
    document.getElementById('editSection').style.display = 'none';
    currentImageId = null;
}

// 删除图片
async function deleteImage(id, isDeleted) {
    const confirmMessage = isDeleted ? '确定要永久删除这张图片吗？此操作不可恢复！' : '确定要标记删除这张图片吗？';
    if (!confirm(confirmMessage)) {
        return;
    }
    
    try {
        // 显示加载状态
        showMessage(`正在${isDeleted ? '永久删除' : '标记删除'}图片...`, 'info');
        
        // 构造请求数据（符合服务器期望的格式）
        let requestData;
        
        // 注意：服务器只支持 'add', 'check', 'update', 'isdel' 四个操作类型
        // 真实删除和标记删除都使用 'isdel' 操作，通过不同的配置来区分
        requestData = {
            table_name: 'resources',
            operation: 'isdel',
            data: {},                 // 重要：data字段是必填的，不能为空
            where_conditions: [
                {
                    field: 'id',
                    operator: '=',
                    value: id
                }
            ],
            soft_delete_config: {
                field: 'is_del',
                value: 'true'
            }
        };
        
        // 发送请求
        const response = await fetch(`${API_BASE_URL}/isdel`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP错误! 状态: ${response.status}`);
        }
        
        const result = await response.json();
        
        if (result.success) {
            showMessage(`图片${isDeleted ? '永久删除' : '标记删除'}成功!`, 'success');
            loadImages();
            // 如果当前显示的是被删除的图片，关闭详情
            if (currentImageId === id) {
                hideImageDetail();
                cancelEdit();
            }
        } else {
            throw new Error(result.message || '删除失败');
        }
    } catch (error) {
        showMessage(`删除图片失败: ${error.message}`, 'error');
        console.error('删除图片错误:', error);
    }
}

// 格式化日期时间
function formatDateTime(dateString) {
    if (!dateString) return 'N/A';
    
    const date = new Date(dateString);
    if (isNaN(date.getTime())) return 'Invalid Date';
    
    return date.toLocaleString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
    });
}

// 测试数据写入（全局函数）
window.testAddData = async function() {
    try {
        // 获取输入值
        const tableNameInput = document.getElementById('testTableName');
        const dataJsonInput = document.getElementById('testDataJson');
        
        const tableName = tableNameInput.value.trim();
        const dataJsonStr = dataJsonInput.value.trim();
        
        if (!tableName) {
            showMessage('请输入表名', 'error');
            return;
        }
        
        if (!dataJsonStr) {
            showMessage('请输入JSON数据', 'error');
            return;
        }
        
        // 解析JSON数据
        let data;
        try {
            data = JSON.parse(dataJsonStr);
        } catch (parseError) {
            showMessage('JSON格式错误', 'error');
            return;
        }
        
        // 显示加载状态
        showMessage('正在测试数据写入...', 'info');
        
        // 构造请求数据
        const requestData = {
            table_name: tableName,
            operation: 'add',
            data: data
        };
        
        // 发送请求
        const response = await fetch(`${API_BASE_URL}/add`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP错误! 状态: ${response.status}`);
        }
        
        const result = await response.json();
        
        if (result.success) {
            showMessage('数据写入成功!', 'success');
            // 清空输入
            tableNameInput.value = '';
            dataJsonInput.value = '';
        } else {
            throw new Error(result.message || '写入失败');
        }
    } catch (error) {
        showMessage(`写入失败: ${error.message}`, 'error');
        console.error('测试数据写入错误:', error);
    }
}