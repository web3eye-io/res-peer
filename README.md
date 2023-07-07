![image](webui/src/assets/ResPeer@3x.png)

# ResPeer: Peer-to-Peer content publishing application on Linera

[![Test](https://github.com/web3eye-io/res-peer/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/web3eye-io/res-peer/actions/workflows/test.yml)

## ResPeer on Linera

ResPeer is a Peer-to-Peer content publishing application on Linera. ResPeer aims to build a community in which author can publish content to get credits easily and the credits can be used to buy assets. Basically, there're two types producer in ResPeer, one is the content producer who publishes content, they won't get any native token incentive directly, but they get credits when they publish content, and when user react to their content, another is asset product who create digital arts then put on shelves of the mall as assets, the asset's price will be set with Linera. A initial credit supply will be set when the application is deploy to Linera. The credits of each reaction will be exponential decay. Each credit amount earned by user reaction has its alive time, and will be destroyed (or return to current total supply) when expired. If the credit balance is lower than threshold, the total supply will be increased with 5% of the initial supply. ResPeer provides a mall for asset producers to put their work on shelves. Producer sets price of their work with Linera. The mall have a unique setting which define the exchange rate of the credits and Linera token. When user buy assets, if they have credits, they can pay to the work with Linera token and credits.

## Workflow

![image](webui/src/assets/Workflow.png)

## Feed of ResPeer

ResPeer provides feed application as an on-chain CMS which will record the content directory of the feed. It also maintain the user reaction to contents. When user action happen, feed application will call credit application to mint credits for sender. This invocation happens on-chain.

User can like, dislike or comment to a content. It's hard to recognize if it's a spam user. So at the first stage, each sender can only like, dislike and comment to a content one time, and each sender can only react to content one time within one minute. User can also reward the content author with gift, credits or Linera token. In future we may introduce some decentralized review mechanism to avoid spam.

## Credit of ResPeer

ResPeer provides credits to incentive user who help to keep the network active. User can earn credits through

* Publish content to feed
* Like or dislike the content
* Comment content
  * The comment is also content, but with less weight to the network, so post a comment earn less credits compare to an origin content

Basically, credits are invaluable in the network. It's valuable only when the mall set exchange rate between the credit and the Linera token. After that when user buy asset from the mall, they can pay with Linera Token, combined with credits.

Each credits amount has its alive time. User's balance will only contain amounts which is not expired. We have two options to the expired credits: return to total supply, or burn it. But it's a bit early to decide which one we should fulfill due to we still lack of enough thoughts about the use cases.

## Mall of ResPeer

ResPeer provides a mall to producers. Beside publishing content, producer can also put their works on shelves with Linera token price. Exchange rate between credits and Linera token will be set through DAO. After that when user want to buy assets from the mall, they can pay with Linera token, combined with credits if they already earn some.

