use smallvec::alloc::sync::Arc;
use crate::types::NodeId;

pub type IResult<T, E = PaxosError> = std::result::Result<T, E>;

#[derive(Debug, Error, Clone)]
pub enum PaxosError {
    #[error("Storage error: {0}")]
    Storage(Arc<anyhow::Error>),

    #[error("This Raft has already been initialized.")]
    AlreadyInitialized,

    #[error("Node '{0}' is unregistered.")]
    UnknownNode(NodeId),

    #[error("Node '{0}' is already registered.")]
    NodeAlreadyRegistered(NodeId),

    #[error("Forward to leader: {0:?}")]
    ForwardToLeader(Option<NodeId>),

    #[error("Shutdown")]
    Shutdown,
}

impl PaxosError {
    pub(crate) fn storage(err: anyhow::Error) -> Self {
        Self::Storage(Arc::new(err))
    }

    #[inline]
    pub(crate) fn is_shutdown(&self) -> bool {
        matches!(self,PaxosError::Shutdown)
    }
}
