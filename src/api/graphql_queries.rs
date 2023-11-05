pub const SEARCH_GQL: &str = r#"
        query(
            $search: SearchInput
            $limit: Int
            $page: Int
            $translationType: VaildTranslationTypeEnumType
            $countryOrigin: VaildCountryOriginEnumType
        ) {
            shows(
                search: $search
                limit: $limit
                page: $page
                translationType: $translationType
                countryOrigin: $countryOrigin
            ) {
                edges {
                    _id name
                    availableEpisodes __typename
                }
            }
        }"#;

pub const EPISODES_LIST_GQL: &str = r#"
query ($showId: String!) {
    show(_id: $showId) {
        _id availableEpisodesDetail
    }
}"#;

pub const EPISODE_EMBED_GQL: &str = r#"
query(
    $showId: String!,
    $translationType: VaildTranslationTypeEnumType!,
    $episodeString: String!
) {
    episode(
        showId: $showId
        translationType: $translationType
        episodeString: $episodeString
    ) {
        episodeString sourceUrls
    }
}"#;
