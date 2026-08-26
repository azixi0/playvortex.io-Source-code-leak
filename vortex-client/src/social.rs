// -- leaked by @azixi0 on github
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportCategory { Username, Bio, Harassment, HateSpeech, Dating, Spam, Cheating, Other }

impl ReportCategory {
    pub fn api_value(self) -> &'static str {
        match self {
            Self::Username => "username", Self::Bio => "bio", Self::Harassment => "harassment",
            Self::HateSpeech => "hate_speech", Self::Dating => "dating", Self::Spam => "spam",
            Self::Cheating => "cheating", Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationshipAction { Follow, FollowBack, Unfollow, AcceptFriendRequest, SendFriendRequest, CancelFriendRequest, Unfriend }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report { pub other_id: u64, pub category: ReportCategory, pub context: String }

