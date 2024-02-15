# Millennium Run (ver 0.10.1)

[한국어](#개요) </br>
[English (Translation)](#overview) </br>

# 개요
Nexon Games에서 제작한 [Blue Archive](https://bluearchive.nexon.com/)의 <b>팬 제작 2차 창작 게임 개발 프로젝트</b> 입니다.
Blue Archive의 2차 창작 가이드라인을 준수하며, \"Nexon Company\", \"Nexon Games\" 또는 \"Yostar\"에서 제작 중단 요청이 있을 경우 이 프로젝트는 파기 됩니다.

# 실행 요구 사양
|Windows|Linux|macOS| 
|:---:|:---:|:---:|
|<b>DirectX 12</b>를 지원하는 그래픽스 하드웨어|<b>Vulkan</b>을 지원하는 그래픽스 하드웨어|<b>Metal</b>을 지원하는 그래픽스 하드웨어|

- 공통 요구사항: BC7 텍스처 압축 포맷을 지원하는 그래픽스 하드웨어

### 실행 확인 표
|Windows|Linux|macOS|
|:---:|:---:|:---:|
|✅|❓|✅|

# 개발 현황
- 한국어 지원
- 게임 인트로, 메뉴, 게임 플레이 구현 완료
- 게임 설정, 저장, 튜토리얼 구현 완료

# 해야 할 목록
- 게임 스테이지 이미지 리소스 확보 (진행중)
- ~~세이브 파일 구현~~ (완료)
- ~~게임 설정 구현~~ (완료)
- ~~튜토리얼 구현~~ (완료)
- 다국어 지원
- 기타 요소 추가...

# 발견된 문제
- `[2024/02/15]` `Windows`에서 사용자가 기본 소리 출력 장치를 변경해도 이전 장치에서 소리가 계속 출력됩니다.
- `[2024/02/15]` `Windows`에서 사용자가 기본 소리 출력 장치를 제거했다가 다시 연결했을 때 소리가 나지 않습니다.

# 라이선스
- <b>이 게임은 \"Blue Archive\" 2차 창작 가이드라인을 준수합니다.</b>
- 이 게임의 [저작자](https://github.com/HK416), \"Nexon Company\", \"Nexon Games\", \"Yostar\" 이외의 다른 사람이나 단체가 상업적으로 이용하는 것을 금지합니다.
- 이 게임의 저작자 및 출처를 표시한 복제 및 배포를 할 수 있습니다.
- 이 게임의 소스 코드는 MIT 라이선스가 부여됩니다.

</br></br>

# Overview
<b>A fan-made secondary creation game development project</b> of [Blue Archive](https://bluearchive.nexon.com/), created by Nexon Games. It complies with the Blue Archive's secondary creation guidelines, and if \"Nexon Company\", \"Nexon Games\" or \"Yostar\" requests to stop production, this project will be destroyed.

# System Requirements
|Windows|Linux|macOS|
|:---:|:---:|:---:|
|Graphics hardware that supports <b>DirectX 12</b>|Graphics hardware that supports <b>Vulkan</b>|Graphics hardware that supports <b>Metal</b>

- Common requirements: Graphics hardware that supports BC7 texture compression format.

### Execution check table
|Windows|Linux|macOS|
|:---:|:---:|:---:|
|✅|❓|✅|

# Development status
- Korean language support
- Completed implementation of game intro, menu, and gameplay
- Completed game setup, save, and tutorial implementation

# TODO list
- Securing game stage image resources (in progress)
- ~~Save file implementation~~ (complete)
- ~~Implementing game settings~~ (complete)
- ~~Tutorial Implementation~~ (complete)
- Multilingual support
- Add other elements...

# Issue
- `[02/15/2024]` In `Windows`, even if the user changes the default sound output device, sound continues to be output from the previous device. 
- `[02/15/2024]` In `Windows`, if the user removes and reconnects the default sound output device, no sound is output.

# License 
- <b>This game complies with the \"Blue Archive\" secondary creation guidelines.</b>
- Commercial use by any person or organization other than the [author of this game](https://github.com/HK416), \"Nexon Company\", \"Nexon Games\", or \"Yostar\" is prohibited.
- Anyone may reproduce and distribute this game with attribution to the author and source.
- The source code for this game is licensed under the MIT License.
