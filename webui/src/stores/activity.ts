import { defineStore } from 'pinia'

export enum ActivityType {
  MeetUp = 'MeetUp',
  Campaign = 'Campaign'
}

export enum VoteType {
  Account = 'Account',
  Power = 'Power'
}

export enum ObjectType {
  Content = 'Content',
  Comment = 'Comment',
  Author = 'Author',
  Reviewer = 'Reviewer',
  ArtWork = 'ArtWork',
  ArtCollection = 'ArtCollection',
  Creator = 'Creator'
}

export interface ObjectCondition {
  class?: Array<string>
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

export interface Winner {
  place: number
  object_id: string
}

export interface Activity {
  id: number
  slogan?: string
  banner: string
  posters: Array<string>
  introduction: string
  host: string
  createdAt: number
  activityType: ActivityType
  votable: boolean
  vote_type: VoteType
  object_type: ObjectType
  object_candidates: Map<string, boolean>
  condition: ObjectCondition
  sponsors: Array<string>
  prizeConfigs: Array<PrizeConfig>
  announcements: Array<string>
  prizeAnnouncement: string
  voter_reward_percent: number
  vote_powers: Map<string, string>
  voters: Map<string, Map<string, boolean>>
  budgetAmount: string
  joinType: JoinType
  location: string
  comments: Array<string>
  registers: Array<string>
  register_start_at: number
  register_end_at: number
  vote_start_at: number
  vote_end_at: number
  participantors: Array<string>
  winners: Array<Winner>
  finalized: boolean
}

export const useActivityStore = defineStore('activity', {
  state: () => ({
    activitiesKeys: [] as Array<number>,
    activities: new Map<string, Activity>()
  }),
  getters: {},
  actions: {}
})
