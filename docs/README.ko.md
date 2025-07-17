# 🔄 hosts-hook [[English](../README.md)]

**hosts-hook**은 시스템의 hosts 파일을 수정하지 않고도 DNS 확인을 재정의할 수 있는 **도구**입니다. 이 도구는 *개발* 및 *테스트 환경*에서 특히 유용합니다.

## ✨ 기능

- 시스템의 DNS 확인 함수를 **후킹**하여 DNS 조회를 가로챕니다.
- 현재 디렉토리 또는 상위 디렉토리에서 사용자 정의 hosts 파일(`.hosts` 또는 `hosts`)을 검색합니다.
- 사용자 정의 hosts 파일에서 일치하는 호스트 이름이 발견되면 실제 DNS 조회 대신 **지정된 IP 주소**를 반환합니다.
- **IPv4** 및 **IPv6** 주소를 모두 지원합니다.
- 환경 변수(`HOSTS_ENV`)를 기반으로 다른 hosts 파일을 사용할 수 있어 다양한 구성(예: *개발*, *프로덕션*)이 가능합니다.

## 💻 지원 플랫폼

- **macOS**
- **Linux**

## 📥 설치

### macOS

macOS에서는 곧 **Homebrew**를 통한 설치를 지원할 예정입니다:

```bash
brew tap powerumc/tap
brew install hostshook
```

### 🛠️ 수동 빌드

저장소를 클론하고 직접 빌드하여 설치할 수도 있습니다:

```bash
# 저장소 클론
git clone https://github.com/powerumc/hosts-hook.git
cd hosts-hook

# 빌드
cargo build
```

## 📚 사용 방법

### 🖥️ CLI 도구 사용하기

Homebrew를 통해 설치된 `hostshook` 바이너리를 사용하여 다음과 같이 hosts 파일을 관리할 수 있습니다:

```bash
# .hosts 파일 초기화
hostshook init

# 특정 이름으로 .hosts 파일 초기화
hostshook init --file .hosts.development
```

### 📚 라이브러리 로드하기

```bash
hostshook
# export DYLD_INSERT_LIBRARIES=/opt/homebrew/lib/libhostshook.dylib
# Or
source <(hostshook)
```

### ⚙️ 환경 변수 설정하기

다른 환경에 대해 다른 hosts 파일을 사용하려면:

```bash
# 환경 변수 설정
export HOSTS_ENV=development

# 이제 .hosts.development 또는 hosts.development 파일을 사용합니다
```

## 🗂️ Hosts 파일 이름

사용자 정의 Hosts 파일은 다음 순서로 검색합니다:
1. `hosts.<environment>` (`HOSTS_ENV` 환경 변수 사용, 예: `HOSTS_ENV=development`)
2. `.hosts.<environment>` (`HOSTS_ENV` 환경 변수 사용, 예: `HOSTS_ENV=development`)
3. `hosts`
4. `.hosts`

현재 디렉토리와 상위 디렉토리에서 다음 순서로 hosts 파일을 검색합니다:

1. 현재 디렉토리에서 Hosts 파일을 찾습니다.
2. 찾지 못한 경우, 루트 디렉토리에 도달할 때까지 Hosts 파일을 찾습니다.
3. 사용자 저의 Hosts 파일을 모두 찾지 못한 경우, 시스템의 기본 Hosts 정보를 사용됩니다.

## 📄 hosts 파일 형식

hosts 파일은 **표준 hosts 파일 형식**을 따릅니다:

```
# 주석
127.0.0.1 example.com
127.0.0.2 example2.com
```

## 🧪 예제

저장소의 `examples` 디렉토리에서 **C 언어** 및 **Node.js** 예제를 확인할 수 있습니다.

### 🔍 C 언어 예제 실행하기

```bash
cd examples/clang
./test.sh
```

아마도 아래와 같은 결과가 출력될 것입니다:
```text
# Before hostshook is loaded
## Testing gethostbyname...
gethostbyname resolved: example.com -> 96.7.128.198

# Hostshook is loaded
## Testing getaddrinfo...
2025-06-11T13:25:32.299Z DEBUG [hostshook] Not found: /..../hosts-hook/examples/clang/hosts
2025-06-11T13:25:32.299Z DEBUG [hostshook] Found: /..../hosts-hook/examples/clang/.hosts
2025-06-11T13:25:32.299Z DEBUG [hostshook] Found IpAddr: 127.0.0.1
2025-06-11T13:25:32.299Z DEBUG [hostshook] Hooked getaddrinfo for: example.com -> 127.0.0.1
getaddrinfo resolved addresses:
  IPv4: 127.0.0.1
```

### 🔍 Node.js 예제 실행하기

```bash
cd examples/nodejs
./test.sh
```

## ⚙️ 작동 방식

**hosts-hook**은 `gethostbyname`, `gethostbyname2`, `getaddrinfo`와 같은 시스템 DNS 확인 함수를 **후킹**하여 DNS 조회를 가로챕니다.
호스트 이름이 사용자 정의 hosts 파일에서 발견되면, 실제 DNS 조회 대신 **지정된 IP 주소**를 반환합니다.
호스트 파일은 현재 디렉토리부터 최상위 디렉토리까지 발견될 때까지 검색됩니다.

## ⚠️ 제한사항

### 🍎 macOS SIP(System Integrity Protection) 제한

macOS에서는 시스템 보안 기능인 SIP(System Integrity Protection)로 인해 일부 제한사항이 있습니다. 시스템 명령어(`ping`, `curl` 등)는 SIP에 의해 보호되어 있어 hosts-hook의 DNS 후킹이 적용되지 않습니다.

이는 macOS의 보안 아키텍처 설계에 따른 것으로, SIP는 시스템 무결성을 보호하기 위해 특정 시스템 프로세스와 명령어가 수정되거나 후킹되는 것을 방지합니다. 따라서 이러한 명령어들은 항상 시스템의 기본 DNS 해석 경로를 사용하게 됩니다.
일반 애플리케이션에서는 hosts-hook이 정상적으로 작동하지만, 시스템 명령어를 사용할 때는 이 제한사항을 염두에 두시기 바랍니다.

## 📜 라이선스

이 프로젝트는 **MIT 라이선스** 하에 배포됩니다.

## 👥 기여

이슈와 풀 리퀘스트는 환영합니다! 🙏
