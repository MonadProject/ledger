# Monadledger

## 开发进度

- [x] 兼容比特币协议的序列化框架
- [x] 集成 Hash256 算法
- [x] 实现 merkle tree 算法
- [ ] 定义核心数据结构（Block Transaction等）(进行中)
- [ ] 兼容比特币协议的P2P通信模块（进行中）
    - [x] 集成异步IO 框架tokio
    - [x] Future: read header
- [ ] 兼容比特币协议的P2P通信协议 (进行中)
    - [ ] commin 数据结构，序列化、反序列化
        - [x] command
        - [ ] inventory_vectors
        - [x] ip_address
        - [x] network_address
        - [x] services
        
    - [x] 定义MessageHeader 数据结构，序列化、反序列化
    - [x] 定义Payload 类型
        - [x] address
        - [x] block
        - [x] feefilter
        - [x] filteradd
        - [x] filterclear
        - [x] filterload
        - [x] get_addr
        - [x] get_blocks
        - [x] get_data
        - [x] headers
        - [x] inv
        - [x] mempool
        - [x] merkleblock
        - [x] not_found
        - [x] ping
        - [x] pong
        - [x] reject
        - [x] sendheaders
        - [x] tx
        - [x] verack
        - [x] version
    - [ ] 为Payload 实现序列化和反序列化方法(宏实现)
       

## 简介

在跟踪了很久 cordaledger 和 hyperledger，以及真实的金融中介的需求之后，莫拉德诺准备从头开始维护自己的一条 side chain，之所以是 side chain 的原因，是因为我们依旧是忠实的公链信仰者，但是就目前的局势，确实没有一条公链能够解决好金融中介复杂的建模需求和频繁的交易需求，但是我们相信未来会有。

总的动机源于，cordaledger 为了数据隔离和安全，牺牲了区块链最终要的一个概念 block，被称之为“无链之链”而备受质疑，虽然设计是合理可靠的，但是和大家对 blockchain 的一般认识格格不入，而 hyperledger，我们了解到在一些 ABS 的场景中，数据隔离性和隐私却又做的不尽如人意。当然二者的 TPS 理所应当表现的很优秀。我们觉得最重要的一点是，脱离了公链基础设施，在安全上是没有说服力的，更不用提价值锚定和转移。

其次，由于 cordaledger 的出发点是专门为金融中介

因此，monadledger 在设计上不仅仅要兼顾两个联盟链的优点，最重要的突破应该是在系统层面上支持诸如 plasma 之类的协议，原生去支持一些知名公链的锚定，而不是从 dapp 层面去 hack。


## 核心概念

### 网络 network

#### 网络基础

#### 门卫 door man

参考 hyperledger 和 cordaledger 的网络设计，monadledger 也被设计成半私密的对等网络，原则上每一个业务节点都需要一个明确的身份，结合现有的 PKI 标准，每一个网络都拥有一个 CA 机构 door man，为每一个进入网络的节点颁布证书。通常情况下，该 door man 需要依赖网络外部的知名 CA 以获得更高的安全性。

### 节点 node

#### 业务节点 business node

业务节点通常根据不同的业务场景，部署多个智能合约 smart contract，在 monadledger 中，智能合约被设计成语言无关，微服务化、且与业务节点完全解耦。业务节点通常运行着实际的业务代码，并持久化相应的状态数据。通常，一项具体的业务场景和需求会被抽象成一个智能合约，并被部署到相应的业务节点上。

#### 记账节点 bookkeeping node

记账节点承载着 monadledger 网络的出块功能。记账节点会周期性的收集网络中的交易（通常是从交易池），验证合法性之后，将其打包进区块。与公链系统 bitcoin 和 ethereum 不同，monadledger 网络中（通常）不存在一条唯一的区块链，而是按照不同的 smart contract 进行状态分片。总的来说，每一个 smart contract 在部署的时候，都会有一个对应的创世区块 genisis block 生成。

#### 验证节点 verify node

记账节点承载着账本实际的出块一职，但记账节点本身并非一成不变，而是基于某种共识算法从验证节点中选举得出。算法可以跟随网络的可信程度进行热拔插，通常来说，当验证节点完全可靠时，采用 PAXOS 或者 RAFT 共识算法，当验证节点可能存在恶意节点时，采用拜占庭容错 PBFT 算法，但拜占庭容错通常只能容忍最多 1/3 的恶意节点。当验证节点未选举上记账节点时，只是被动的接受区块广播并对其进行验证、记账。验证节点可以独立部署，同时也可以和业务节点合并部署。

#### Oracle 节点

Oracle 节点作为连通区块链世界和现实世界的桥梁，承担着将现实世界中的“真实信息”（如当前的天气温度、利率等）带到 monadledger 中，供其他节点或者 dapp 进行消费的责任。Oracle 节点通常由被整个网络可信的组织部署，免费或者收费的对网络中其他节点提供服务。

#### 服务发现 service discovery

### 状态 state

状态 state 作为 monadledger 中最重要、最基础的建模工具，从货币模型扩展而来，代表着节点每时每刻能查到的账本数据（有价证券）。状态通常被设计成不可变对象，一旦生成就不可被更改、而只能被消费。通常一个交易（价值交易）会有0到多个 state 对象作为输入，同时会有0个或多个 state 对象作为输出。state 可以是任意的结构化数据或者非结构化数据，使其拥有表达股权、债务等能力。

### 数据 data

数据 data 则是 monadledger 中另一个较为普遍的建模工具，表达的是单纯的数据对象 pure data，通常数据是结构化的，基本上等价于关系型数据库中的表，由交易（数据交易）对其进行演变。


### 交易 transaction

交易是对于更新账本数据的一次提案，任何节点都可以在任何时刻都可以对网络发起一笔交易，初始化的交易将短暂的存在于网络的交易池中（通常是验证节点和记账节点的内存中），并在某个时刻被打包进入区块。与 cordaledger 的交易设计显著不同的是，monadledger 中交易被显示的区分为价值交易 value transaction 和数据交易 data transaction。前者遵循 UTXO 模型，适用于有价证券的转移，而后者遵循 Account 模型，适用于纯数据的事务性更新。

#### 价值交易 value transaction

价值交易遵循 UTXO 模型，其建模对象为 state，即价值交易会消费0个或多个 state 的同时，生成0个或者多个 state。该类型的交易在进入交易池和即将被打包进区块时会进行如下验证 1）交易不存在“双花” 2）拥有必要的参与方及其签名。验证失败的交易则会被立即丢弃。

#### 数据交易 data transaction

数据交易遵循 Account 模型，其建模对象为 data,即数据交易会事务性的更改 data 的值。该类型的交易在进入交易池和即将被打包进区块时会进行如下验证 1）所要修改的 data 存在（首次创建除外）2）拥有必要的参与方及其签名。验证失败的交易则会被立即丢弃。

### 智能合约 smart contract

智能合约等价于 cordaledger 中的 dapp 以及 hyperledger 中的 chaincode，描述的是运行在 monadledger 中的去中心化应用，表达了实际的业务需求。在 monadledger 中，智能合约通常被设计成与平台无关（支持虚拟机和容器化环境），语言无关（golang,java或者python等任意编程语言），微服务化（通常一个智能合约可以看做一个或者多个微服务的集合），且与节点完全解耦（通常运行在节点之外的进程）。同时，monadledger 支持安全、友好的智能合约升级机制。智能合约在首次部署时，会向网络发起一笔特殊的 bootstrap 交易，该交易会被网络单独打包进一个特殊的区块，即创世区块。因此，monadledger 中会有多个创世区块，也意味着同时存在多条区块链。

### 区块 block

为了必要的数据安全和隔离的需要，monadledger 根据不同的 smart contract 对状态进行分片（sharding,通常情况下，公链项目进行 sharding 的动机是为了提升 TPS，以满足 dapp 的实际需求），即每个 smart contract 对应一条特定的区块链 origin blockchain，该链的所有区块只会打包由该 smart contract 发起的交易。于此同时，monad ledger 依旧维护着一条包含所有交易的完整的区块链 root blockchain。

#### 大块 big block

大块是构成 root blockchain 的基本元素，包含任意类型的交易，包括那些由不同 smart contract 发起的交易。root blockchain 拥有一个特殊的大块，即创世大块，在网络首次选举出记账节点时创建。

#### 原始块 origin block

原始块是构成 origin blockchain 的基本元素，只包含由某个特定的 smart contract 发起的交易。origin blockchain 由多条，同样包含一个特殊的原始块，即创世原始块，在 smart contract 首次部署时创建。

### 共识算法 consensus algorithm

共识算法 consensus algorithm 维护着整个账本数据的一致性，对系统的安全和可靠行显得尤为重要。monadledger 采用可拔插的共识算法机制，系统可以随着节点的可信程度的变化选择不同的共识算法。通常，如果网络中所有节点都是可信并可控的，则可以使用 RAFT 进行共识，如果一旦网络被放开，可能的作恶节点会加入到网络，则可以采用拜占庭容错算法 PBFT。

### 公链锚定、扩容和侧链支持

monadledger 从系统层面上原生支持对知名公链（bitcoin 和 ethereum）的锚定，以获得更高的安全性；同时 monadledger 也计划集成闪电网络协议和 plasma 协议，为公链的扩容方案提供友好支持。

## 应用场景 Application

### 数据存证和版权保护

### 资产证券化 ABS

### 供应链管理和供应链金融

