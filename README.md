# 仓库拣货路径规划模拟器

基于 **TypeScript + Tauri + React + Rust** 的仓库拣货路径规划模拟器，提供真实仓储调度后台体验。

## 功能特性

### 前端 (TypeScript + React)
- 📋 **订单列表** - 优先级排序、状态筛选、进度展示
- 🗺️ **仓库网格地图** - 可视化货架、拣货点、规划路径
- 🛤️ **路线展示** - 详细拣货步骤、当前位置高亮
- 📊 **统计面板** - 实时订单/批次统计、效率评分

### 后端 (Rust)
- 🔍 **A* 路径规划** - 货架障碍绕行、最短路径计算
- 📦 **TSP 优化** - 多拣货点最优顺序规划
- 🎯 **订单调度** - 优先级调度（紧急/高/中/低）
- 🔄 **智能合并** - 多订单合并、重复商品合并
- ⚖️ **容量分批** - 按重量/数量限制分批
- ⚠️ **异常处理** - 缺货标记、损坏记录

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | React 18 + TypeScript |
| 构建工具 | Vite 5 |
| 桌面框架 | Tauri 1 |
| 后端语言 | Rust 1.7+ |
| 路径算法 | A* + 最近邻 TSP |

## 环境要求

- **Node.js** >= 16.0.0
- **Rust** >= 1.70.0 (通过 rustup 安装)
- **Visual Studio Build Tools** (Windows) - 包含 "C++ 桌面开发" 工作负载

### Windows 环境准备

1. **安装 Rust**
   ```powershell
   # 访问 https://rustup.rs/ 下载并运行 rustup-init.exe
   # 或使用 winget
   winget install Rustlang.Rustup
   ```

2. **安装 Visual Studio Build Tools**
   ```
   下载地址: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   安装时勾选: "使用 C++ 的桌面开发"
   ```

3. **安装 Node.js**
   ```powershell
   winget install OpenJS.NodeJS.LTS
   ```

## 安装步骤

### 1. 安装前端依赖

```powershell
npm install
```

### 2. 安装 Tauri CLI (全局)

```powershell
cargo install tauri-cli
```

### 3. 生成应用图标 (首次运行需要)

Tauri 需要特定格式的图标文件。首次运行前需要生成：

```powershell
# 方法1: 如果有一张源图片 (PNG/SVG 124x124 以上)
cargo tauri icon path/to/your/icon.png

# 方法2: 使用默认占位图标
# 项目已配置为使用默认图标，开发模式下可能不需要手动生成
```

### 4. 开发模式运行

```powershell
cargo tauri dev

# 或使用 npm 脚本
npm run tauri dev
```

> 💡 **首次运行** 会自动下载并编译 Rust 依赖，可能需要 5-10 分钟（取决于网络和机器配置）。

## 使用流程

1. **初始化测试数据**
   - 点击顶部工具栏的 **"🧪 初始化测试数据"**
   - 系统会加载 7 组预设测试订单和 20 种商品

2. **创建拣货批次**
   - 点击 **"📦 创建批次"**
   - 系统自动执行：
     - 按优先级排序订单
     - 合并相同商品
     - 按容量限制分批
     - 计算最优拣货路径

3. **开始拣货**
   - 在右侧 **批次列表** 中选择一个批次
   - 点击 **"▶️ 开始拣货"**
   - 观察地图上的规划路径（蓝色线）

4. **执行拣货**
   - 底部出现 **拣货操作面板**
   - 可操作选项：
     - **确认拣货** - 手动输入数量
     - **全部拣完** - 一键完成当前商品
     - **标记异常** - 如缺货、损坏等

5. **观察实时更新**
   - 🟣 紫色 = 待拣货
   - 🟢 绿色 = 已拣货
   - 🔴 红色闪烁 = 当前位置
   - 🔵 蓝色 = 规划路径

## 项目结构

```
CK/
├── src/                          # TypeScript 前端
│   ├── components/
│   │   ├── OrdersPanel.tsx       # 订单列表面板
│   │   ├── MapPanel.tsx          # 仓库地图组件
│   │   ├── RoutePanel.tsx        # 路线展示面板
│   │   ├── StatsPanel.tsx        # 统计面板
│   │   ├── BatchPanel.tsx        # 批次列表面板
│   │   └── PickingPanel.tsx      # 拣货操作面板
│   ├── App.tsx                   # 主应用
│   ├── api.ts                    # Tauri API 封装
│   ├── types.ts                  # 类型定义
│   ├── main.tsx                  # 入口文件
│   └── index.css                 # 全局样式
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs               # Tauri 入口
│   │   ├── lib.rs                # 模块导出
│   │   ├── models.rs             # 数据模型
│   │   ├── scheduler.rs          # 核心调度器
│   │   ├── pathfinding.rs        # A* 路径算法
│   │   └── warehouse.rs          # 仓库管理
│   ├── Cargo.toml                # Rust 依赖
│   ├── tauri.conf.json           # Tauri 配置
│   └── icons/                    # 应用图标
├── package.json
├── tsconfig.json
└── vite.config.ts
```

## 预设测试订单

| 订单号 | 优先级 | 商品数量 | 用途 |
|--------|--------|----------|------|
| ORD-2024001 | 紧急 | 4 | 高优先级测试 |
| ORD-2024002 | 高 | 4 | 正常优先级 |
| ORD-2024003 | 中 | 4 | 正常优先级 |
| ORD-2024004 | 低 | 4 | 低优先级 |
| ORD-2024005 | 高 | 2 | 商品合并测试 |
| ORD-2024006 | 高 | 2 | 商品合并测试 |
| ORD-2024007 | 中 | 4 | 大容量分批测试 |

## 常见问题

### Q: 首次运行卡住怎么办？
A: 首次运行需要下载 Rust 依赖和编译，请确保网络连接正常，耐心等待 5-10 分钟。

### Q: 提示 "failed to run tauri-build"
A: 确保已安装 Visual Studio Build Tools 并勾选了 "C++ 桌面开发"。

### Q: 图标相关错误
A: 开发模式下图标不是必需的。如果报错，尝试：
```powershell
# 创建 icons 目录
mkdir src-tauri\icons
# 或注释掉 tauri.conf.json 中的 icon 配置
```

### Q: 如何构建生产版本？
```powershell
cargo tauri build
```
输出位于 `src-tauri/target/release/bundle/`

## License

MIT
