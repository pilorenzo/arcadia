use std::collections::HashSet;

use arcadia_shared::tracker::models::peer_id::PeerId;

pub fn is_torrent_client_allowed(
    peer_id: &PeerId,
    allowed_torrent_clients: &HashSet<Vec<u8>>,
) -> bool {
    let peer_id_without_hyphen = &peer_id.0[1..];
    for prefix in allowed_torrent_clients.iter() {
        if peer_id_without_hyphen.starts_with(prefix) {
            return true;
        }
    }
    false
}
