# Claw Proxy

여러 AI 프로바이더를 단일 로컬 프록시로 통합하는 크로스 플랫폼 개발자 도구.

## 주요 기능

- **OpenAI + Anthropic 호환** — `/v1/chat/completions`과 `/v1/messages` 엔드포인트 모두 지원
- **멀티 프로바이더 라우팅** — RoundRobin, Failover 전략
- **실시간 로그** — WebSocket으로 요청 로그 스트리밍
- **YAML 설정** — hot reload 지원
- **메뉴바 앱** — macOS 시스템 트레이에서 아이콘 클릭으로 사이드바 창 표시

## 빠른 시작

### 1. 코어 서버 실행

```bash
cp configs/config.example.yaml config.yaml
# config.yaml에 실제 API 키 입력
cargo run -p claw-proxy-core -- config.yaml
```

### 2. 요청 테스트

```bash
# OpenAI 형식
curl http://localhost:47380/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"gpt-4o","messages":[{"role":"user","content":"hi"}]}'

# Anthropic 형식 (Claude Code 연동)
export ANTHROPIC_BASE_URL=http://localhost:47380
```

### 3. Claude Code 연동

```bash
export ANTHROPIC_BASE_URL=http://localhost:47380
claude  # 기존 명령 그대로 사용
```

## 포트

| 포트   | 역할                  |
|--------|-----------------------|
| 47380  | 프록시 (OpenAI + Anthropic) |
| 47381  | 관리 API (REST)       |
| 47382  | WebSocket (로그)      |

## 설정

`config.yaml`:

```yaml
server:
  proxy_port: 47380
  admin_port: 47381
  ws_port: 47382

routing:
  strategy: round_robin  # round_robin | failover

providers:
  - name: openai-account-1
    type: openai
    api_key: sk-...
    models: ["gpt-4o", "gpt-4o-mini"]
```

## 개발

### 코어 빌드

```bash
cargo build -p claw-proxy-core
```

### 전체 테스트

```bash
cargo test --workspace
```

### 데스크탑 앱 (Tauri)

```bash
cd apps/desktop
./build-core.sh    # 코어 바이너리 빌드
npm install        # 의존성 설치
npm run tauri dev  # 개발 모드
```

## 아키텍처

```
클라이언트 → :47380 (프록시)
              ↓
         [Format Detector]
              ↓
         [Normalizer] → InternalRequest
              ↓
         [Router] → Provider 선택
              ↓
         [Provider Adapter] → 외부 API
              ↓
         [Response Normalizer] → 원래 형식으로 변환
              ↓
         클라이언트 응답

Tauri UI ←→ :47381 (REST)
Tauri UI  ←  :47382 (WebSocket)
```
