![image](webui/src/assets/ResPeer@3x.png)

# ResPeer: Peer-to-Peer content publishing application on Linera

[![Test](https://github.com/web3eye-io/res-peer/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/web3eye-io/res-peer/actions/workflows/test.yml)

## ResPeer on Linera

ResPeer is a Peer-to-Peer content publishing application on Linera. ResPeer aims to build a community in which author can publish content to get credits easily and the credits can be used to buy assets. Basically, there're two types producer in ResPeer, one is the content producer who publishes content, they won't get any native token incentive directly, but they get credits when they publish content, and when user react to their content, another is asset product who create digital arts then put on shelves of the mall as assets, the asset's price will be set with Linera. A initial credit supply will be set when the application is deploy to Linera. The credits of each reaction will be exponential decay. Each credit amount earned by user reaction has its alive time, and will be destroyed (or return to current total supply) when expired. If the credit balance is lower than threshold, the total supply will be increased with 5% of the initial supply. ResPeer provides a mall for asset producers to put their work on shelves. Producer sets price of their work with Linera. The mall have a unique setting which define the exchange rate of the credits and Linera token. When user buy assets, if they have credits, they can pay to the work with Linera token and credits.

## Workflow

![image](webui/src/assets/Workflow.png)