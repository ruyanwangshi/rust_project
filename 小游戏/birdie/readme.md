# Agenda项目

1. 建立项目
2. 实现Game loop
3. 不同的游戏模式
4. 添加玩家
5. 添加障碍和计分
6. 汇总

## Game loop
>
> 为了让游戏流畅、顺滑的运行，需要使用Game loop

Game loop:

1. 初始化窗口、图形和其它资源；
2. 每当屏幕刷新（通常是每秒30、60或更多次），它都会云南行；
3. 每次通过循环，它都会调用游戏的tick() 函数；

## Bracket-Lib

> Brack-Lib是一个Rust游戏编程库：
> 作为简单的教学工具
> 抽象了游戏开发很多复杂的东西
> 保留了相关的概念

Bracket-Lib包括很多库：
> 随机数生成、几何、路径寻找、颜色处理、常用算法等。

## Bracket-terminal

bracket-terminal是Bracket-Lib中负责显示的部分

1. 提供了模拟控制台
2. 可以与多种平台配合
3. 从文本控制台到Web Assembly、 例如 OpenGL、Vulkan、Metal
4. 支持sprites和原生OpennGL开发

## Codepage 437：IBM扩展ASCII字符集

codepage 437：

1. 来自Dos PC上的字符，用于终端输出，除了字符和数字，还提供了一些符号。
2. Bracket-lib会把字符翻译成图形sprites并提供一个有限的字符集，字符所展示的是相应的图片。


## 游戏模式
1. 游戏通常在不同的模式中运行
2. 每种模式会明确游戏在当前的tick()中应该做什么
> 我们这个游戏需要三种模式：
> 菜单
> 游戏中
> 结束
