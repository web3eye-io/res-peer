import { defineStore } from 'pinia'

export enum ActivityType {
  MeetUp = 'MeetUp',
  Campaign = 'Campaign'
}

export const ActivityTypes = Object.values(ActivityType)

export enum VoteType {
  Account = 'Account',
  Power = 'Power'
}

export const VoteTypes = Object.values(VoteType)

export enum ObjectType {
  Content = 'Content',
  Comment = 'Comment',
  Author = 'Author',
  Reviewer = 'Reviewer',
  ArtWork = 'ArtWork',
  ArtCollection = 'ArtCollection',
  Creator = 'Creator'
}

export const ObjectTypes = Object.values(ObjectType)

export interface ObjectCondition {
  classes?: Array<string>
  minWords: number
  maxWords: number
}

export interface PrizeConfig {
  place: number
  medal: string
  title: string
  rewardAmount?: string
}

export enum JoinType {
  Online = 'Online',
  InPerson = 'InPerson'
}

export const JoinTypes = Object.values(JoinType)

export interface Winner {
  place: number
  object_id: string
}

export interface CreateParams {
  title: string
  slogan?: string
  banner: string
  posters: Array<string>
  introduction: string
  activityType: ActivityType
  votable: boolean
  voteType: VoteType
  objectType: ObjectType
  condition: ObjectCondition
  sponsors: Array<string>
  prizeConfigs: Array<PrizeConfig>
  voterRewardPercent: number
  budgetAmount: string
  joinType: JoinType
  location: string
  registerStartAt: number
  registerEndAt: number
  voteStartAt: number
  voteEndAt: number
}

export interface Activity {
  id: number
  title: string
  slogan?: string
  banner: string
  posters: Array<string>
  introduction: string
  host: string
  createdAt: number
  activityType: ActivityType
  votable: boolean
  voteType: VoteType
  objectType: ObjectType
  objectCandidates: Map<string, boolean>
  condition: ObjectCondition
  sponsors: Array<string>
  prizeConfigs: Array<PrizeConfig>
  announcements: Map<string, boolean>
  prizeAnnouncement: string
  voterRewardPercent: number
  votePowers: Map<string, string>
  voters: Map<string, Map<string, boolean>>
  budgetAmount: string
  joinType: JoinType
  location: string
  comments: Array<string>
  registers: Array<string>
  registerStartAt: number
  registerEndAt: number
  voteStartAt: number
  voteEndAt: number
  participantors: Array<string>
  winners: Array<Winner>
  finalized: boolean
}

export const useActivityStore = defineStore('activity', {
  state: () => ({
    activitiesKeys: [] as Array<number>,
    activities: new Map<number, Activity>()
  }),
  getters: {
    _activities (): (host?: string) => Array<Activity> {
      return (host?: string) => {
        return Array.from(this.activities.values()).filter((el) => {
          let ok = true
          if (host) ok &&= el.host === host
          return ok
        }).sort((a, b) => a.createdAt - b.createdAt)
      }
    }
  },
  actions: {}
})
