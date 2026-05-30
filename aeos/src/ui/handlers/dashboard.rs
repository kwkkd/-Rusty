use axum::response::Html;

pub async fn dashboard() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

const DASHBOARD_HTML: &str = r#"
<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AEOS - AI Agent Operating System</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #1e3a8a 0%, #0f172a 100%);
            color: #e2e8f0;
            min-height: 100vh;
        }
        
        .container {
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }
        
        header {
            background: rgba(30, 58, 138, 0.5);
            backdrop-filter: blur(10px);
            padding: 20px;
            border-radius: 12px;
            margin-bottom: 30px;
            border: 1px solid rgba(148, 163, 184, 0.2);
        }
        
        h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
            background: linear-gradient(135deg, #60a5fa 0%, #3b82f6 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        
        .status-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        
        .card {
            background: rgba(30, 58, 138, 0.3);
            border: 1px solid rgba(148, 163, 184, 0.2);
            border-radius: 12px;
            padding: 20px;
            backdrop-filter: blur(10px);
            transition: all 0.3s ease;
        }
        
        .card:hover {
            background: rgba(30, 58, 138, 0.5);
            border-color: rgba(148, 163, 184, 0.4);
        }
        
        .card-title {
            font-size: 1.2em;
            margin-bottom: 15px;
            color: #60a5fa;
        }
        
        .metric {
            display: flex;
            justify-content: space-between;
            margin-bottom: 12px;
            padding: 8px 0;
            border-bottom: 1px solid rgba(148, 163, 184, 0.1);
        }
        
        .metric:last-child {
            border-bottom: none;
        }
        
        .metric-label {
            color: #cbd5e1;
        }
        
        .metric-value {
            color: #60a5fa;
            font-weight: 600;
        }
        
        .progress-bar {
            background: rgba(0, 0, 0, 0.3);
            border-radius: 4px;
            height: 8px;
            margin-top: 8px;
            overflow: hidden;
        }
        
        .progress-fill {
            background: linear-gradient(90deg, #60a5fa 0%, #3b82f6 100%);
            height: 100%;
            transition: width 0.3s ease;
        }
        
        .button {
            background: linear-gradient(135deg, #60a5fa 0%, #3b82f6 100%);
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 6px;
            cursor: pointer;
            transition: all 0.3s ease;
            margin-top: 15px;
            width: 100%;
        }
        
        .button:hover {
            transform: translateY(-2px);
            box-shadow: 0 10px 20px rgba(96, 165, 250, 0.3);
        }
        
        .agents-section {
            margin-top: 30px;
        }
        
        .agents-list {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 15px;
        }
        
        .agent-item {
            background: rgba(50, 100, 200, 0.2);
            border: 1px solid rgba(96, 165, 250, 0.3);
            border-radius: 8px;
            padding: 15px;
            transition: all 0.3s ease;
        }
        
        .agent-item:hover {
            background: rgba(50, 100, 200, 0.3);
            border-color: rgba(96, 165, 250, 0.6);
        }
        
        .agent-name {
            font-weight: 600;
            color: #60a5fa;
            margin-bottom: 8px;
        }
        
        .agent-status {
            font-size: 0.9em;
            color: #94a3b8;
        }
        
        .status-badge {
            display: inline-block;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 0.85em;
            margin-left: 8px;
        }
        
        .status-running {
            background: rgba(34, 197, 94, 0.2);
            color: #22c55e;
        }
        
        .status-idle {
            background: rgba(148, 163, 184, 0.2);
            color: #94a3b8;
        }
        
        footer {
            text-align: center;
            margin-top: 40px;
            color: #64748b;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🤖 AEOS</h1>
            <p>AI Agent Operating System - Advanced Management Platform</p>
        </header>
        
        <div class="status-grid">
            <div class="card">
                <div class="card-title">시스템 상태</div>
                <div class="metric">
                    <span class="metric-label">Uptime:</span>
                    <span class="metric-value" id="uptime">1h 2m</span>
                </div>
                <div class="metric">
                    <span class="metric-label">Kernel:</span>
                    <span class="metric-value">v0.1.0</span>
                </div>
                <div class="metric">
                    <span class="metric-label">Status:</span>
                    <span class="metric-value">🟢 Running</span>
                </div>
            </div>
            
            <div class="card">
                <div class="card-title">CPU 사용량</div>
                <div class="metric">
                    <span class="metric-label">Usage:</span>
                    <span class="metric-value" id="cpu-usage">35%</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-fill" id="cpu-progress" style="width: 35%"></div>
                </div>
                <button class="button">상세 보기</button>
            </div>
            
            <div class="card">
                <div class="card-title">메모리 사용량</div>
                <div class="metric">
                    <span class="metric-label">Used:</span>
                    <span class="metric-value" id="mem-usage">4GB / 8GB</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-fill" id="mem-progress" style="width: 50%"></div>
                </div>
                <button class="button">상세 보기</button>
            </div>
        </div>
        
        <div class="agents-section">
            <div class="card">
                <div class="card-title">에이전트 목록</div>
                <div class="agents-list" id="agents-list">
                    <div class="agent-item">
                        <div class="agent-name">Agent-001 <span class="status-badge status-running">Running</span></div>
                        <div class="agent-status">Tasks: 5 | CPU: 15% | Mem: 256MB</div>
                    </div>
                    <div class="agent-item">
                        <div class="agent-name">Agent-002 <span class="status-badge status-idle">Idle</span></div>
                        <div class="agent-status">Tasks: 0 | CPU: 0% | Mem: 128MB</div>
                    </div>
                    <div class="agent-item">
                        <div class="agent-name">Agent-003 <span class="status-badge status-running">Running</span></div>
                        <div class="agent-status">Tasks: 3 | CPU: 8% | Mem: 192MB</div>
                    </div>
                </div>
                <button class="button">새 에이전트 생성</button>
            </div>
        </div>
        
        <footer>
            <p>AEOS v0.1.0 | Powered by Rust & Tokio</p>
        </footer>
    </div>
    
    <script>
        // Update metrics every 2 seconds
        setInterval(async () => {
            try {
                const response = await fetch('/api/system/resources');
                const data = await response.json();
                
                document.getElementById('cpu-usage').textContent = Math.round(data.cpu_usage) + '%';
                document.getElementById('cpu-progress').style.width = data.cpu_usage + '%';
                
                const memPercent = (data.memory_used / data.memory_total) * 100;
                const memGb = (data.memory_used / (1024 * 1024 * 1024)).toFixed(1);
                const totalGb = (data.memory_total / (1024 * 1024 * 1024)).toFixed(1);
                document.getElementById('mem-usage').textContent = memGb + 'GB / ' + totalGb + 'GB';
                document.getElementById('mem-progress').style.width = memPercent + '%';
            } catch (error) {
                console.error('Failed to fetch metrics:', error);
            }
        }, 2000);
    </script>
</body>
</html>
"#;
