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
  objectId: string
}

export interface CreateParams {
  title: string
  slogan?: string
  banner: string
  hostResume: string
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
  registerStartAt: string
  registerEndAt: string
  voteStartAt: string
  voteEndAt: string
}

export interface Activity {
  id: number
  title: string
  slogan?: string
  banner: string
  posters: Array<string>
  introduction: string
  host: string
  hostResume: string
  createdAt: number
  activityType: ActivityType
  votable: boolean
  voteType: VoteType
  objectType: ObjectType
  objectCandidates: Array<string>
  condition: ObjectCondition
  sponsors: Array<string>
  prizeConfigs: Array<PrizeConfig>
  announcements: Array<string>
  prizeAnnouncement: string
  voterRewardPercent: number
  votePowers: Map<string, string>
  voters: Map<string, Array<string>>
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
        }).sort((a, b) => b.createdAt - a.createdAt)
      }
    },
    votes (): (id: number) => number {
      return (id: number) => {
        let votes = 0
        Object.values(this.activities.get(id)?.voters || new Map<string, Array<string>>()).forEach((el: Array<string>) => {
          votes += el.length
        })
        return votes
      }
    },
    objectCandidateCount (): (id: number) => number {
      return (id: number) => {
        return (this.activities.get(id)?.objectCandidates || []).length
      }
    },
    objectCandidates (): (id: number) => Array<string> {
      return (id: number) => {
        return this.activities.get(id)?.objectCandidates || []
      }
    },
    objectVotePower (): (id: number, objectId: string) => number {
      return (id: number, objectId: string) => {
        return Number(new Map(Object.entries(this.activities.get(id)?.votePowers || new Map<string, string>())).get(objectId)) || 0
      }
    },
    objectVoteCount (): (id: number, objectId: string) => number {
      return (id: number, objectId: string) => {
        return Number(
          (new Map<string, Array<string>>(Object.entries(this.activities.get(id)?.voters || new Map<string, Array<string>>())).get(objectId) || []).length)
      }
    },
    objectWon (): (id: number, objectId: string) => boolean {
      return (id: number, objectId: string) => {
        return this.activities.get(id)?.winners?.find((el) => el.objectId === objectId) !== undefined
      }
    },
    activity (): (id: number) => Activity | undefined {
      return (id: number) => {
        return this.activities.get(id)
      }
    },
    activityType (): (id: number) => ActivityType {
      return (id: number) => {
        let activityType = this.activities.get(id)?.activityType || ActivityType.Campaign
        switch (activityType) {
          case ActivityType.Campaign.toUpperCase():
            activityType = ActivityType.Campaign
            break
          case ActivityType.MeetUp.toUpperCase():
            activityType = ActivityType.MeetUp
            break
        }
        return activityType
      }
    },
    voteType (): (id: number) => VoteType {
      return (id: number) => {
        let voteType = this.activities.get(id)?.voteType || VoteType.Power
        switch (voteType) {
          case VoteType.Account.toUpperCase():
            voteType = VoteType.Account
            break
          case VoteType.Power.toUpperCase():
            voteType = VoteType.Power
            break
        }
        return voteType
      }
    },
    objectType (): (id: number) => ObjectType {
      return (id: number) => {
        let objectType = this.activities.get(id)?.objectType || ObjectType.Content
        switch (objectType) {
          case ObjectType.ArtCollection.toUpperCase():
            objectType = ObjectType.ArtCollection
            break
          case ObjectType.ArtWork.toUpperCase():
            objectType = ObjectType.ArtWork
            break
          case ObjectType.Author.toUpperCase():
            objectType = ObjectType.Author
            break
          case ObjectType.Comment.toUpperCase():
            objectType = ObjectType.Comment
            break
          case ObjectType.Content.toUpperCase():
            objectType = ObjectType.Content
            break
          case ObjectType.Creator.toUpperCase():
            objectType = ObjectType.Creator
            break
          case ObjectType.Reviewer.toUpperCase():
            objectType = ObjectType.Reviewer
            break
        }
        return objectType
      }
    },
    joinType (): (id: number) => JoinType {
      return (id: number) => {
        let joinType = this.activities.get(id)?.joinType || JoinType.Online
        switch (joinType) {
          case JoinType.Online.toUpperCase():
            joinType = JoinType.Online
            break
          case JoinType.InPerson.toUpperCase():
            joinType = JoinType.InPerson
            break
        }
        return joinType
      }
    },
    objectRegistered (): (id: number, objectId: string) => boolean | undefined {
      return (id: number, objectId: string) => {
        return (this.activities.get(id)?.objectCandidates || []).includes(objectId)
      }
    },
    objectVoted (): (id: number, objectId: string, account: string) => boolean | undefined {
      return (id: number, objectId: string, account: string) => {
        const voters = new Map<string, Map<string, boolean>>(Object.entries(this.activities.get(id)?.voters || new Map<string, Array<string>>()))
        return ((voters.get(objectId) || []) as Array<string>).includes(account)
      }
    }
  },
  actions: {}
})
