#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidAppDeliveryData {
    #[prost(int64, optional, tag = "1")]
    pub download_size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub sha1: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub download_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub additional_file: ::prost::alloc::vec::Vec<AppFileMetadata>,
    #[prost(message, repeated, tag = "5")]
    pub download_auth_cookie: ::prost::alloc::vec::Vec<HttpCookie>,
    #[prost(bool, optional, tag = "6")]
    pub forward_locked: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "7")]
    pub refund_timeout: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "8", default = "true")]
    pub server_initiated: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "9")]
    pub post_install_refund_window_millis: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "10")]
    pub immediate_start_needed: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "11")]
    pub patch_data: ::core::option::Option<AndroidAppPatchData>,
    #[prost(message, optional, tag = "12")]
    pub encryption_params: ::core::option::Option<EncryptionParams>,
    #[prost(string, optional, tag = "13")]
    pub compressed_download_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "14")]
    pub compressed_size: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "15")]
    pub split_delivery_data: ::prost::alloc::vec::Vec<SplitDeliveryData>,
    #[prost(int32, optional, tag = "16")]
    pub install_location: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "17")]
    pub r#type: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "18")]
    pub compressed_app_data: ::core::option::Option<CompressedAppData>,
    #[prost(string, optional, tag = "19")]
    pub sha256: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitDeliveryData {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2")]
    pub download_size: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub compressed_size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "4")]
    pub sha1: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub download_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub compressed_download_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub patch_data: ::core::option::Option<AndroidAppPatchData>,
    #[prost(message, optional, tag = "8")]
    pub compressed_app_data: ::core::option::Option<CompressedAppData>,
    #[prost(string, optional, tag = "9")]
    pub sha256: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidAppPatchData {
    #[prost(int32, optional, tag = "1")]
    pub base_version_code: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub base_sha1: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub download_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4", default = "1")]
    pub patch_format: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "5")]
    pub max_patch_size: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompressedAppData {
    #[prost(int64, optional, tag = "1")]
    pub r#type: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub download_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppFileMetadata {
    #[prost(int32, optional, tag = "1")]
    pub file_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub version_code: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "3")]
    pub size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "4")]
    pub download_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub patch_data: ::core::option::Option<AndroidAppPatchData>,
    #[prost(int64, optional, tag = "6")]
    pub compressed_size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub compressed_download_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub sha1: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionParams {
    #[prost(int32, optional, tag = "1")]
    pub version: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub encryption_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub h_mac_key: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpCookie {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub address_line1: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub address_line2: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub state: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub postal_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub postal_country: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub dependent_locality: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub sorting_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "12")]
    pub deprecated_is_reduced: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "13")]
    pub first_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub last_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrowseLink {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub data_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "5")]
    pub icon: ::core::option::Option<Image>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrowseResponse {
    #[prost(string, optional, tag = "1")]
    pub contents_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub promo_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub category: ::prost::alloc::vec::Vec<BrowseLink>,
    #[prost(message, repeated, tag = "4")]
    pub breadcrumb: ::prost::alloc::vec::Vec<BrowseLink>,
    #[prost(message, repeated, tag = "5")]
    pub quick_link: ::prost::alloc::vec::Vec<QuickLink>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "7")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "8")]
    pub backend_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "9")]
    pub browse_tab: ::core::option::Option<BrowseTab>,
    #[prost(int32, optional, tag = "10")]
    pub landing_tab_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub quick_link_tab_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub quick_link_fallback_tab_index: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "14")]
    pub is_family_safe: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "18")]
    pub share_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectPurchase {
    #[prost(string, optional, tag = "1")]
    pub details_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub purchase_item_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub parent_item_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4", default = "1")]
    pub offer_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemGiftCard {
    #[prost(string, optional, tag = "1")]
    pub prefill_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub partner_payload: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolvedLink {
    #[prost(string, optional, tag = "1")]
    pub details_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub browse_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub search_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub direct_purchase: ::core::option::Option<DirectPurchase>,
    #[prost(string, optional, tag = "5")]
    pub home_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub redeem_gift_card: ::core::option::Option<RedeemGiftCard>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "8")]
    pub doc_id: ::core::option::Option<DocId>,
    #[prost(string, optional, tag = "9")]
    pub wishlist_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "10")]
    pub backend: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "11")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub my_account_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "13")]
    pub help_center: ::core::option::Option<HelpCenter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelpCenter {
    #[prost(string, optional, tag = "1")]
    pub context_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuickLink {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<Image>,
    #[prost(message, optional, tag = "3")]
    pub link: ::core::option::Option<ResolvedLink>,
    #[prost(bool, optional, tag = "4")]
    pub display_required: ::core::option::Option<bool>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "6")]
    pub backend_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "7")]
    pub prism_style: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrowseTab {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "3")]
    pub list_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub browse_link: ::prost::alloc::vec::Vec<BrowseLink>,
    #[prost(message, repeated, tag = "5")]
    pub quick_link: ::prost::alloc::vec::Vec<QuickLink>,
    #[prost(string, optional, tag = "6")]
    pub quick_link_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub categories_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "8")]
    pub backend_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "9")]
    pub highlights_banner_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyResponse {
    #[prost(message, optional, tag = "1")]
    pub purchase_response: ::core::option::Option<PurchaseNotificationResponse>,
    #[prost(group, optional, tag = "2")]
    pub checkoutinfo: ::core::option::Option<buy_response::CheckoutInfo>,
    #[prost(string, optional, tag = "8")]
    pub continue_via_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub purchase_status_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub checkout_service_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "13")]
    pub checkout_token_required: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "14")]
    pub base_checkout_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "37")]
    pub tos_checkbox_html: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "38")]
    pub iab_permission_error: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "39")]
    pub purchase_status_response: ::core::option::Option<PurchaseStatusResponse>,
    #[prost(string, optional, tag = "46")]
    pub purchase_cookie: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "49")]
    pub challenge: ::core::option::Option<Challenge>,
    #[prost(string, optional, tag = "50")]
    pub add_instrument_prompt_html: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "51")]
    pub confirm_button_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "52")]
    pub permission_error_title_text: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "53")]
    pub permission_error_message_text: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(bytes = "vec", optional, tag = "54")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "55")]
    pub encoded_delivery_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "56")]
    pub unknown_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `BuyResponse`.
pub mod buy_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CheckoutInfo {
        #[prost(message, optional, tag = "3")]
        pub item: ::core::option::Option<super::LineItem>,
        #[prost(message, repeated, tag = "4")]
        pub sub_item: ::prost::alloc::vec::Vec<super::LineItem>,
        #[prost(group, repeated, tag = "5")]
        pub checkoutoption: ::prost::alloc::vec::Vec<checkout_info::CheckoutOption>,
        #[prost(string, optional, tag = "10")]
        pub deprecated_checkout_url: ::core::option::Option<
            ::prost::alloc::string::String,
        >,
        #[prost(string, optional, tag = "11")]
        pub add_instrument_url: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "20")]
        pub footer_html: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(int32, repeated, packed = "false", tag = "31")]
        pub eligible_instrument_family: ::prost::alloc::vec::Vec<i32>,
        #[prost(string, repeated, tag = "36")]
        pub footnote_html: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "44")]
        pub eligible_instrument: ::prost::alloc::vec::Vec<super::Instrument>,
    }
    /// Nested message and enum types in `CheckoutInfo`.
    pub mod checkout_info {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CheckoutOption {
            #[prost(string, optional, tag = "6")]
            pub form_of_payment: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "7")]
            pub encoded_adjusted_cart: ::core::option::Option<
                ::prost::alloc::string::String,
            >,
            #[prost(string, optional, tag = "15")]
            pub instrument_id: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(message, repeated, tag = "16")]
            pub item: ::prost::alloc::vec::Vec<super::super::LineItem>,
            #[prost(message, repeated, tag = "17")]
            pub sub_item: ::prost::alloc::vec::Vec<super::super::LineItem>,
            #[prost(message, optional, tag = "18")]
            pub total: ::core::option::Option<super::super::LineItem>,
            #[prost(string, repeated, tag = "19")]
            pub footer_html: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            #[prost(int32, optional, tag = "29")]
            pub instrument_family: ::core::option::Option<i32>,
            #[prost(int32, repeated, packed = "false", tag = "30")]
            pub deprecated_instrument_inapplicable_reason: ::prost::alloc::vec::Vec<i32>,
            #[prost(bool, optional, tag = "32")]
            pub selected_instrument: ::core::option::Option<bool>,
            #[prost(message, optional, tag = "33")]
            pub summary: ::core::option::Option<super::super::LineItem>,
            #[prost(string, repeated, tag = "35")]
            pub footnote_html: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            #[prost(message, optional, tag = "43")]
            pub instrument: ::core::option::Option<super::super::Instrument>,
            #[prost(string, optional, tag = "45")]
            pub purchase_cookie: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, repeated, tag = "48")]
            pub disabled_reason: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineItem {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub offer: ::core::option::Option<Offer>,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Money {
    #[prost(int64, optional, tag = "1")]
    pub micros: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub formatted_amount: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseNotificationResponse {
    #[prost(int32, optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub debug_info: ::core::option::Option<DebugInfo>,
    #[prost(string, optional, tag = "3")]
    pub localized_error_message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub purchase_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseStatusResponse {
    #[prost(int32, optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub status_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub status_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub brief_message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub info_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub library_update: ::core::option::Option<LibraryUpdate>,
    #[prost(message, optional, tag = "7")]
    pub rejected_instrument: ::core::option::Option<Instrument>,
    #[prost(message, optional, tag = "8")]
    pub app_delivery_data: ::core::option::Option<AndroidAppDeliveryData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseHistoryDetails {
    #[prost(int64, optional, tag = "2")]
    pub purchase_timestamp_millis: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub purchase_details_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub offer: ::core::option::Option<Offer>,
    #[prost(string, optional, tag = "6")]
    pub purchase_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub title_byline_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub client_refund_context: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "9")]
    pub purchase_details_image: ::core::option::Option<Image>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingProfileResponse {
    #[prost(int32, optional, tag = "1")]
    pub result: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub billing_profile: ::core::option::Option<BillingProfile>,
    #[prost(string, optional, tag = "3")]
    pub user_message_html: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckInstrumentResponse {
    #[prost(bool, optional, tag = "1")]
    pub user_has_valid_instrument: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub checkout_token_required: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "4")]
    pub instrument: ::prost::alloc::vec::Vec<Instrument>,
    #[prost(message, repeated, tag = "5")]
    pub eligible_instrument: ::prost::alloc::vec::Vec<Instrument>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentSetupInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub setup_info: ::prost::alloc::vec::Vec<InstrumentSetupInfo>,
    #[prost(bool, optional, tag = "2")]
    pub checkout_token_required: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemGiftCardRequest {
    #[prost(string, optional, tag = "1")]
    pub gift_card_pin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub address: ::core::option::Option<Address>,
    #[prost(string, repeated, tag = "3")]
    pub accepted_legal_document_id: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "4")]
    pub checkout_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemGiftCardResponse {
    #[prost(int32, optional, tag = "1")]
    pub result: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub user_message_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub balance_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub address_challenge: ::core::option::Option<AddressChallenge>,
    #[prost(bool, optional, tag = "5")]
    pub checkout_token_required: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstrumentRequest {
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Instrument>,
    #[prost(string, optional, tag = "2")]
    pub checkout_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstrumentResponse {
    #[prost(int32, optional, tag = "1")]
    pub result: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub instrument_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub user_message_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub error_input_field: ::prost::alloc::vec::Vec<InputValidationError>,
    #[prost(bool, optional, tag = "5")]
    pub checkout_token_required: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "6")]
    pub redeemed_offer: ::core::option::Option<RedeemedPromoOffer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitiateAssociationResponse {
    #[prost(string, optional, tag = "1")]
    pub user_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyAssociationResponse {
    #[prost(int32, optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub billing_address: ::core::option::Option<Address>,
    #[prost(message, optional, tag = "3")]
    pub carrier_tos: ::core::option::Option<CarrierTos>,
    #[prost(string, optional, tag = "4")]
    pub carrier_error_message: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressChallenge {
    #[prost(string, optional, tag = "1")]
    pub response_address_param: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub response_checkboxes_param: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "3")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub checkbox: ::prost::alloc::vec::Vec<FormCheckbox>,
    #[prost(message, optional, tag = "6")]
    pub address: ::core::option::Option<Address>,
    #[prost(message, repeated, tag = "7")]
    pub error_input_field: ::prost::alloc::vec::Vec<InputValidationError>,
    #[prost(string, optional, tag = "8")]
    pub error_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed = "false", tag = "9")]
    pub required_field: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "10")]
    pub supported_country: ::prost::alloc::vec::Vec<Country>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationChallenge {
    #[prost(int32, optional, tag = "1")]
    pub authentication_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub response_authentication_type_param: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "3")]
    pub response_retry_count_param: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "4")]
    pub pin_header_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub pin_description_text_html: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "6")]
    pub gaia_header_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub gaia_description_text_html: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "8")]
    pub gaia_footer_text_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "9")]
    pub gaia_opt_out_checkbox: ::core::option::Option<FormCheckbox>,
    #[prost(string, optional, tag = "10")]
    pub gaia_opt_out_description_text_html: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Challenge {
    #[prost(message, optional, tag = "1")]
    pub address_challenge: ::core::option::Option<AddressChallenge>,
    #[prost(message, optional, tag = "2")]
    pub authentication_challenge: ::core::option::Option<AuthenticationChallenge>,
    #[prost(message, optional, tag = "3")]
    pub web_view_challenge: ::core::option::Option<WebViewChallenge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Country {
    #[prost(string, optional, tag = "1")]
    pub region_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormCheckbox {
    #[prost(string, optional, tag = "1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub checked: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub required: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputValidationError {
    #[prost(int32, optional, tag = "1")]
    pub input_field: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub error_message: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebViewChallenge {
    #[prost(string, optional, tag = "1")]
    pub start_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub target_url_regexp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub cancel_button_display_label: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "4")]
    pub response_target_url_param: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "5")]
    pub cancel_url_regexp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCreditCardPromoOffer {
    #[prost(string, optional, tag = "1")]
    pub header_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub description_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub image: ::core::option::Option<Image>,
    #[prost(string, optional, tag = "4")]
    pub introductory_text_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub offer_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub no_action_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub terms_and_conditions_html: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailablePromoOffer {
    #[prost(message, optional, tag = "1")]
    pub add_credit_card_offer: ::core::option::Option<AddCreditCardPromoOffer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPromoOfferResponse {
    #[prost(message, repeated, tag = "1")]
    pub available_offer: ::prost::alloc::vec::Vec<AvailablePromoOffer>,
    #[prost(message, optional, tag = "2")]
    pub redeemed_offer: ::core::option::Option<RedeemedPromoOffer>,
    #[prost(bool, optional, tag = "3")]
    pub checkout_token_required: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemedPromoOffer {
    #[prost(string, optional, tag = "1")]
    pub header_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub description_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub image: ::core::option::Option<Image>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocId {
    #[prost(string, optional, tag = "1")]
    pub backend_doc_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2", default = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub backend: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Install {
    #[prost(fixed64, optional, tag = "1")]
    pub android_id: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "2")]
    pub version: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub bundled: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub pending: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "5")]
    pub last_updated: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLicenseKey {
    #[prost(fixed64, optional, tag = "1")]
    pub dasher_customer_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "2")]
    pub doc_id: ::core::option::Option<DocId>,
    #[prost(int32, optional, tag = "3", default = "1")]
    pub licensed_offer_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub rental_period_days: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LicenseTerms {
    #[prost(message, optional, tag = "1")]
    pub group_license_key: ::core::option::Option<GroupLicenseKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Offer {
    #[prost(int64, optional, tag = "1")]
    pub micros: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub formatted_amount: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub converted_price: ::prost::alloc::vec::Vec<Offer>,
    #[prost(bool, optional, tag = "5")]
    pub checkout_flow_required: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "6")]
    pub full_price_micros: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub formatted_full_amount: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "8", default = "1")]
    pub offer_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "9")]
    pub rental_terms: ::core::option::Option<RentalTerms>,
    #[prost(int64, optional, tag = "10")]
    pub on_sale_date: ::core::option::Option<i64>,
    #[prost(string, repeated, tag = "11")]
    pub promotion_label: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "12")]
    pub subscription_terms: ::core::option::Option<SubscriptionTerms>,
    #[prost(string, optional, tag = "13")]
    pub formatted_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub formatted_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "15")]
    pub preorder: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "16")]
    pub on_sale_date_display_time_zone_offset_millis: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "17")]
    pub licensed_offer_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "18")]
    pub subscription_content_terms: ::core::option::Option<SubscriptionContentTerms>,
    #[prost(string, optional, tag = "19")]
    pub offer_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "20")]
    pub preorder_fulfillment_display_date: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "21")]
    pub license_terms: ::core::option::Option<LicenseTerms>,
    #[prost(bool, optional, tag = "22")]
    pub sale: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "23")]
    pub voucher_terms: ::core::option::Option<VoucherTerms>,
    #[prost(message, repeated, tag = "24")]
    pub offer_payment: ::prost::alloc::vec::Vec<OfferPayment>,
    #[prost(bool, optional, tag = "25")]
    pub repeat_last_payment: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "26")]
    pub buy_button_label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "27")]
    pub instant_purchase_enabled: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "30")]
    pub sale_end_timestamp: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "31")]
    pub sale_message: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthAndDay {
    #[prost(uint32, optional, tag = "1")]
    pub month: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub day: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferPaymentPeriod {
    #[prost(message, optional, tag = "1")]
    pub duration: ::core::option::Option<TimePeriod>,
    #[prost(message, optional, tag = "2")]
    pub start: ::core::option::Option<MonthAndDay>,
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<MonthAndDay>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferPaymentOverride {
    #[prost(int64, optional, tag = "1")]
    pub micros: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "2")]
    pub start: ::core::option::Option<MonthAndDay>,
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<MonthAndDay>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferPayment {
    #[prost(int64, optional, tag = "1")]
    pub micros: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub offer_payment_period: ::core::option::Option<OfferPaymentPeriod>,
    #[prost(message, repeated, tag = "4")]
    pub offer_payment_override: ::prost::alloc::vec::Vec<OfferPaymentOverride>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoucherTerms {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RentalTerms {
    #[prost(int32, optional, tag = "1")]
    pub d_eprecated_grant_period_seconds: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub d_eprecated_activate_period_seconds: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub grant_period: ::core::option::Option<TimePeriod>,
    #[prost(message, optional, tag = "4")]
    pub activate_period: ::core::option::Option<TimePeriod>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedData {
    #[prost(string, optional, tag = "1")]
    pub signed_data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub signature: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionContentTerms {
    #[prost(message, optional, tag = "1")]
    pub required_subscription: ::core::option::Option<DocId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupLicenseInfo {
    #[prost(int32, optional, tag = "1")]
    pub licensed_offer_type: ::core::option::Option<i32>,
    #[prost(fixed64, optional, tag = "2")]
    pub gaia_group_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LicensedDocumentInfo {
    #[prost(fixed64, repeated, packed = "false", tag = "1")]
    pub gaia_group_id: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnershipInfo {
    #[prost(int64, optional, tag = "1")]
    pub initiation_timestamp: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub valid_until_timestamp: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "3")]
    pub auto_renewing: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "4")]
    pub refund_timeout_timestamp: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "5")]
    pub post_delivery_refund_window_millis: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "6")]
    pub developer_purchase_info: ::core::option::Option<SignedData>,
    #[prost(bool, optional, tag = "7")]
    pub pre_ordered: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub hidden: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "9")]
    pub rental_terms: ::core::option::Option<RentalTerms>,
    #[prost(message, optional, tag = "10")]
    pub group_license_info: ::core::option::Option<GroupLicenseInfo>,
    #[prost(message, optional, tag = "11")]
    pub licensed_document_info: ::core::option::Option<LicensedDocumentInfo>,
    #[prost(int32, optional, tag = "12")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "14")]
    pub library_expiration_timestamp: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionTerms {
    #[prost(message, optional, tag = "1")]
    pub recurring_period: ::core::option::Option<TimePeriod>,
    #[prost(message, optional, tag = "2")]
    pub trial_period: ::core::option::Option<TimePeriod>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimePeriod {
    #[prost(int32, optional, tag = "1")]
    pub unit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingAddressSpec {
    #[prost(int32, optional, tag = "1")]
    pub billing_address_type: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub required_field: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingProfile {
    #[prost(message, repeated, tag = "1")]
    pub instrument: ::prost::alloc::vec::Vec<Instrument>,
    #[prost(string, optional, tag = "2")]
    pub selected_external_instrument_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "3")]
    pub billing_profile_option: ::prost::alloc::vec::Vec<BillingProfileOption>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingProfileOption {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub display_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub external_instrument_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub topup_info: ::core::option::Option<TopupInfo>,
    #[prost(message, optional, tag = "5")]
    pub carrier_billing_instrument_status: ::core::option::Option<
        CarrierBillingInstrumentStatus,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarrierBillingCredentials {
    #[prost(string, optional, tag = "1")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2")]
    pub expiration: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarrierBillingInstrument {
    #[prost(string, optional, tag = "1")]
    pub instrument_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub account_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub transaction_limit: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub subscriber_identifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub encrypted_subscriber_info: ::core::option::Option<EncryptedSubscriberInfo>,
    #[prost(message, optional, tag = "7")]
    pub credentials: ::core::option::Option<CarrierBillingCredentials>,
    #[prost(message, optional, tag = "8")]
    pub accepted_carrier_tos: ::core::option::Option<CarrierTos>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarrierBillingInstrumentStatus {
    #[prost(message, optional, tag = "1")]
    pub carrier_tos: ::core::option::Option<CarrierTos>,
    #[prost(bool, optional, tag = "2")]
    pub association_required: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub password_required: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "4")]
    pub carrier_password_prompt: ::core::option::Option<PasswordPrompt>,
    #[prost(int32, optional, tag = "5")]
    pub api_version: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub device_association: ::core::option::Option<DeviceAssociation>,
    #[prost(string, optional, tag = "8")]
    pub carrier_support_phone_number: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarrierTos {
    #[prost(message, optional, tag = "1")]
    pub dcb_tos: ::core::option::Option<CarrierTosEntry>,
    #[prost(message, optional, tag = "2")]
    pub pii_tos: ::core::option::Option<CarrierTosEntry>,
    #[prost(bool, optional, tag = "3")]
    pub needs_dcb_tos_acceptance: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub needs_pii_tos_acceptance: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarrierTosEntry {
    #[prost(string, optional, tag = "1")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditCardInstrument {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub escrow_handle: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub last_digits: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub expiration_month: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub expiration_year: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "6")]
    pub escrow_efe_param: ::prost::alloc::vec::Vec<EfeParam>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceAssociation {
    #[prost(string, optional, tag = "1")]
    pub user_token_request_message: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "2")]
    pub user_token_request_address: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisabledInfo {
    #[prost(int32, optional, tag = "1")]
    pub disabled_reason: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub disabled_message_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub error_message: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EfeParam {
    #[prost(int32, optional, tag = "1")]
    pub key: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instrument {
    #[prost(string, optional, tag = "1")]
    pub instrument_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub billing_address: ::core::option::Option<Address>,
    #[prost(message, optional, tag = "3")]
    pub credit_card: ::core::option::Option<CreditCardInstrument>,
    #[prost(message, optional, tag = "4")]
    pub carrier_billing: ::core::option::Option<CarrierBillingInstrument>,
    #[prost(message, optional, tag = "5")]
    pub billing_address_spec: ::core::option::Option<BillingAddressSpec>,
    #[prost(int32, optional, tag = "6")]
    pub instrument_family: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "7")]
    pub carrier_billing_status: ::core::option::Option<CarrierBillingInstrumentStatus>,
    #[prost(string, optional, tag = "8")]
    pub display_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "9")]
    pub topup_info_deprecated: ::core::option::Option<TopupInfo>,
    #[prost(int32, optional, tag = "10")]
    pub version: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "11")]
    pub stored_value: ::core::option::Option<StoredValueInstrument>,
    #[prost(message, repeated, tag = "12")]
    pub disabled_info: ::prost::alloc::vec::Vec<DisabledInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentSetupInfo {
    #[prost(int32, optional, tag = "1")]
    pub instrument_family: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub supported: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "3")]
    pub address_challenge: ::core::option::Option<AddressChallenge>,
    #[prost(message, optional, tag = "4")]
    pub balance: ::core::option::Option<Money>,
    #[prost(string, repeated, tag = "5")]
    pub footer_html: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordPrompt {
    #[prost(string, optional, tag = "1")]
    pub prompt: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub forgot_password_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredValueInstrument {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<Money>,
    #[prost(message, optional, tag = "3")]
    pub topup_info: ::core::option::Option<TopupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopupInfo {
    #[prost(string, optional, tag = "1")]
    pub options_container_doc_id_deprecated: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "2")]
    pub options_list_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub options_container_doc_id: ::core::option::Option<DocId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumePurchaseResponse {
    #[prost(message, optional, tag = "1")]
    pub library_update: ::core::option::Option<LibraryUpdate>,
    #[prost(int32, optional, tag = "2")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerMetadata {
    #[prost(string, optional, tag = "1")]
    pub browse_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub next_page_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "3")]
    pub relevance: ::core::option::Option<f64>,
    #[prost(int64, optional, tag = "4")]
    pub estimated_results: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub analytics_cookie: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub ordered: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "7")]
    pub container_view: ::prost::alloc::vec::Vec<ContainerView>,
    #[prost(message, optional, tag = "8")]
    pub left_icon: ::core::option::Option<Image>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerView {
    #[prost(bool, optional, tag = "1")]
    pub selected: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub list_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlagContentResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientDownloadRequest {
    #[prost(string, optional, tag = "1")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub digests: ::core::option::Option<client_download_request::Digests>,
    #[prost(int64, optional, tag = "3")]
    pub length: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "4")]
    pub resources: ::prost::alloc::vec::Vec<client_download_request::Resource>,
    #[prost(message, optional, tag = "5")]
    pub signature: ::core::option::Option<client_download_request::SignatureInfo>,
    #[prost(bool, optional, tag = "6")]
    pub user_initiated: ::core::option::Option<bool>,
    #[prost(string, repeated, tag = "8")]
    pub client_asn: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub file_basename: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "10")]
    pub download_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "11")]
    pub locale: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "12")]
    pub apk_info: ::core::option::Option<client_download_request::ApkInfo>,
    #[prost(fixed64, optional, tag = "13")]
    pub android_id: ::core::option::Option<u64>,
    #[prost(string, repeated, tag = "15")]
    pub originating_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "17")]
    pub originating_signature: ::core::option::Option<
        client_download_request::SignatureInfo,
    >,
}
/// Nested message and enum types in `ClientDownloadRequest`.
pub mod client_download_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApkInfo {
        #[prost(string, optional, tag = "1")]
        pub package_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "2")]
        pub version_code: ::core::option::Option<i32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertificateChain {
        #[prost(message, repeated, tag = "1")]
        pub element: ::prost::alloc::vec::Vec<certificate_chain::Element>,
    }
    /// Nested message and enum types in `CertificateChain`.
    pub mod certificate_chain {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Element {
            #[prost(bytes = "vec", optional, tag = "1")]
            pub certificate: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(bool, optional, tag = "2")]
            pub parsed_successfully: ::core::option::Option<bool>,
            #[prost(bytes = "vec", optional, tag = "3")]
            pub subject: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(bytes = "vec", optional, tag = "4")]
            pub issuer: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(bytes = "vec", optional, tag = "5")]
            pub fingerprint: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(int64, optional, tag = "6")]
            pub expiry_time: ::core::option::Option<i64>,
            #[prost(int64, optional, tag = "7")]
            pub start_time: ::core::option::Option<i64>,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Digests {
        #[prost(bytes = "vec", optional, tag = "1")]
        pub sha256: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub sha1: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "3")]
        pub md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        #[prost(string, optional, tag = "1")]
        pub url: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "2")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(bytes = "vec", optional, tag = "3")]
        pub remote_ip: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(string, optional, tag = "4")]
        pub referrer: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SignatureInfo {
        #[prost(message, repeated, tag = "1")]
        pub certificate_chain: ::prost::alloc::vec::Vec<CertificateChain>,
        #[prost(bool, optional, tag = "2")]
        pub trusted: ::core::option::Option<bool>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientDownloadResponse {
    #[prost(int32, optional, tag = "1")]
    pub verdict: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub more_info: ::core::option::Option<client_download_response::MoreInfo>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `ClientDownloadResponse`.
pub mod client_download_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MoreInfo {
        #[prost(string, optional, tag = "1")]
        pub description: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub url: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientDownloadStatsRequest {
    #[prost(int32, optional, tag = "1")]
    pub user_decision: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugInfo {
    #[prost(string, repeated, tag = "1")]
    pub message: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(group, repeated, tag = "2")]
    pub timing: ::prost::alloc::vec::Vec<debug_info::Timing>,
}
/// Nested message and enum types in `DebugInfo`.
pub mod debug_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Timing {
        #[prost(string, optional, tag = "3")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(double, optional, tag = "4")]
        pub time_in_ms: ::core::option::Option<f64>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugSettingsResponse {
    #[prost(string, optional, tag = "1")]
    pub play_country_override: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub play_country_debug_info: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryResponse {
    #[prost(int32, optional, tag = "1", default = "1")]
    pub status: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub app_delivery_data: ::core::option::Option<AndroidAppDeliveryData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkDetailsEntry {
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<Item>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkDetailsRequest {
    #[prost(string, repeated, tag = "1")]
    pub doc_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2", default = "true")]
    pub include_child_docs: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub include_details: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub source_package_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed = "false", tag = "7")]
    pub installed_version_code: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkDetailsResponse {
    #[prost(message, repeated, tag = "1")]
    pub entry: ::prost::alloc::vec::Vec<BulkDetailsEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailsResponse {
    #[prost(string, optional, tag = "2")]
    pub analytics_cookie: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub user_review: ::core::option::Option<Review>,
    #[prost(message, optional, tag = "4")]
    pub item: ::core::option::Option<Item>,
    #[prost(string, optional, tag = "5")]
    pub footer_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "7")]
    pub discovery_badge: ::prost::alloc::vec::Vec<DiscoveryBadge>,
    #[prost(bool, optional, tag = "8", default = "true")]
    pub enable_reviews: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "12")]
    pub features: ::core::option::Option<Features>,
    #[prost(string, optional, tag = "13")]
    pub details_stream_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub user_review_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub post_acquire_details_stream_url: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryBadge {
    #[prost(string, optional, tag = "1")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<Image>,
    #[prost(int32, optional, tag = "3")]
    pub background_color: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub badge_container1: ::core::option::Option<DiscoveryBadgeLink>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "6")]
    pub is_plus_one: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "7")]
    pub aggregate_rating: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "8")]
    pub user_star_rating: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "9")]
    pub download_count: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub download_units: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub content_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "12")]
    pub player_badge: ::core::option::Option<PlayerBadge>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub family_age_range_badge: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "14")]
    pub family_category_badge: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBadge {
    #[prost(message, optional, tag = "1")]
    pub overlay_icon: ::core::option::Option<Image>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryBadgeLink {
    #[prost(message, optional, tag = "1")]
    pub link: ::core::option::Option<Link>,
    #[prost(string, optional, tag = "2")]
    pub user_reviews_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub critic_reviews_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Features {
    #[prost(message, repeated, tag = "1")]
    pub feature_presence: ::prost::alloc::vec::Vec<Feature>,
    #[prost(message, repeated, tag = "2")]
    pub feature_rating: ::prost::alloc::vec::Vec<Feature>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    #[prost(string, optional, tag = "1")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceConfigurationProto {
    #[prost(int32, optional, tag = "1")]
    pub touch_screen: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub keyboard: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub navigation: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub screen_layout: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub has_hard_keyboard: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub has_five_way_navigation: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "7")]
    pub screen_density: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub gl_es_version: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "9")]
    pub system_shared_library: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "10")]
    pub system_available_feature: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(string, repeated, tag = "11")]
    pub native_platform: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "12")]
    pub screen_width: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub screen_height: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "14")]
    pub system_supported_locale: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(string, repeated, tag = "15")]
    pub gl_extension: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "16")]
    pub device_class: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "17", default = "50")]
    pub max_apk_download_size_mb: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "18")]
    pub smallest_screen_width_dp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "19", default = "0")]
    pub low_ram_device: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "20", default = "8354971648")]
    pub total_memory_bytes: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "21", default = "8")]
    pub max_num_of_cpu_cores: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "26")]
    pub device_feature: ::prost::alloc::vec::Vec<DeviceFeature>,
    #[prost(int32, optional, tag = "28", default = "0")]
    pub unknown28: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "30", default = "4")]
    pub unknown30: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceFeature {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub value: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    #[prost(message, optional, tag = "1")]
    pub doc_id: ::core::option::Option<DocId>,
    #[prost(message, optional, tag = "2")]
    pub fetch_doc_id: ::core::option::Option<DocId>,
    #[prost(message, optional, tag = "3")]
    pub sample_doc_id: ::core::option::Option<DocId>,
    #[prost(string, optional, tag = "4")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub snippet: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub price_deprecated: ::core::option::Option<Offer>,
    #[prost(message, optional, tag = "9")]
    pub availability: ::core::option::Option<Availability>,
    #[prost(message, repeated, tag = "10")]
    pub image: ::prost::alloc::vec::Vec<Image>,
    #[prost(message, repeated, tag = "11")]
    pub child: ::prost::alloc::vec::Vec<Document>,
    #[prost(message, optional, tag = "13")]
    pub aggregate_rating: ::core::option::Option<AggregateRating>,
    #[prost(message, repeated, tag = "14")]
    pub offer: ::prost::alloc::vec::Vec<Offer>,
    #[prost(message, repeated, tag = "15")]
    pub translated_snippet: ::prost::alloc::vec::Vec<TranslatedText>,
    #[prost(message, repeated, tag = "16")]
    pub document_variant: ::prost::alloc::vec::Vec<DocumentVariant>,
    #[prost(string, repeated, tag = "17")]
    pub category_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "18")]
    pub decoration: ::prost::alloc::vec::Vec<Document>,
    #[prost(message, repeated, tag = "19")]
    pub parent: ::prost::alloc::vec::Vec<Document>,
    #[prost(string, optional, tag = "20")]
    pub privacy_policy_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    pub consumption_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "22")]
    pub estimated_num_children: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "23")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentVariant {
    #[prost(int32, optional, tag = "1")]
    pub variation_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub rule: ::core::option::Option<Rule>,
    #[prost(string, optional, tag = "3")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub snippet: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub recent_changes: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub auto_translation: ::prost::alloc::vec::Vec<TranslatedText>,
    #[prost(message, repeated, tag = "7")]
    pub offer: ::prost::alloc::vec::Vec<Offer>,
    #[prost(int64, optional, tag = "9")]
    pub channel_id: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "10")]
    pub child: ::prost::alloc::vec::Vec<Document>,
    #[prost(message, repeated, tag = "11")]
    pub decoration: ::prost::alloc::vec::Vec<Document>,
    #[prost(message, repeated, tag = "12")]
    pub image: ::prost::alloc::vec::Vec<Image>,
    #[prost(string, repeated, tag = "13")]
    pub category_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SectionImage {
    #[prost(message, repeated, tag = "1")]
    pub image_container: ::prost::alloc::vec::Vec<ImageContainer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageContainer {
    #[prost(message, optional, tag = "4")]
    pub image: ::core::option::Option<Image>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    #[prost(int32, optional, tag = "1")]
    pub image_type: ::core::option::Option<i32>,
    #[prost(group, optional, tag = "2")]
    pub dimension: ::core::option::Option<image::Dimension>,
    #[prost(string, optional, tag = "5")]
    pub image_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub alt_text_localized: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub secure_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "8")]
    pub position_in_sequence: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "9")]
    pub supports_fife_url_options: ::core::option::Option<bool>,
    #[prost(group, optional, tag = "10")]
    pub citation: ::core::option::Option<image::Citation>,
    #[prost(int32, optional, tag = "14")]
    pub duration_seconds: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "15")]
    pub fill_color_rgb: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "16")]
    pub autogen: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "17")]
    pub attribution: ::core::option::Option<Attribution>,
    #[prost(string, optional, tag = "19")]
    pub background_color_rgb: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "20")]
    pub palette: ::core::option::Option<ImagePalette>,
    #[prost(int32, optional, tag = "21")]
    pub device_class: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "22")]
    pub supports_fife_monogram_option: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "28")]
    pub image_url_alt: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Image`.
pub mod image {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dimension {
        #[prost(int32, optional, tag = "3")]
        pub width: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub height: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "18")]
        pub aspect_ratio: ::core::option::Option<i32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Citation {
        #[prost(string, optional, tag = "11")]
        pub title_localized: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "12")]
        pub url: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribution {
    #[prost(string, optional, tag = "1")]
    pub source_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub source_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub license_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub license_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImagePalette {
    #[prost(string, optional, tag = "1")]
    pub light_vibrant_rgb: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub vibrant_rgb: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub dark_vibrant_rgb: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub light_muted_rgb: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub muted_rgb: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub dark_muted_rgb: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslatedText {
    #[prost(string, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub source_locale: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub target_locale: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlusOneData {
    #[prost(bool, optional, tag = "1")]
    pub set_by_user: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "2")]
    pub total: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub circles_total: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "4")]
    pub circles_people: ::prost::alloc::vec::Vec<PlusPerson>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlusPerson {
    #[prost(string, optional, tag = "2")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub profile_image_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppDetails {
    #[prost(string, optional, tag = "1")]
    pub developer_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub major_version_number: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub version_code: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub version_string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub app_category: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "8")]
    pub content_rating: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "9")]
    pub info_download_size: ::core::option::Option<i64>,
    #[prost(string, repeated, tag = "10")]
    pub permission: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub developer_email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub developer_website: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub info_download: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub package_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub recent_changes_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub info_updated_on: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "17")]
    pub file: ::prost::alloc::vec::Vec<FileMetadata>,
    #[prost(string, optional, tag = "18")]
    pub app_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "19")]
    pub certificate_hash: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "21", default = "true")]
    pub varies_with_device: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "22")]
    pub certificate_set: ::prost::alloc::vec::Vec<CertificateSet>,
    #[prost(string, repeated, tag = "23")]
    pub auto_acquire_free_app_if_higher_version_available_tag: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(bool, optional, tag = "24")]
    pub has_instant_link: ::core::option::Option<bool>,
    #[prost(string, repeated, tag = "25")]
    pub split_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "26")]
    pub gamepad_required: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "27")]
    pub externally_hosted: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "28")]
    pub ever_externally_hosted: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "30")]
    pub install_notes: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "31")]
    pub install_location: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "32")]
    pub target_sdk_version: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "33")]
    pub has_preregistration_promo_code: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "34")]
    pub dependencies: ::core::option::Option<Dependencies>,
    #[prost(message, optional, tag = "35")]
    pub testing_program_info: ::core::option::Option<TestingProgramInfo>,
    #[prost(message, optional, tag = "36")]
    pub early_access_info: ::core::option::Option<EarlyAccessInfo>,
    #[prost(message, optional, tag = "41")]
    pub editor_choice: ::core::option::Option<EditorChoice>,
    #[prost(string, optional, tag = "43")]
    pub instant_link: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "45")]
    pub developer_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "46")]
    pub publisher: ::core::option::Option<Publisher>,
    #[prost(string, optional, tag = "48")]
    pub category_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "53")]
    pub download_count: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "61")]
    pub download_label_display: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "67")]
    pub in_app_product: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "77")]
    pub download_label_abbreviated: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "78")]
    pub download_label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "82")]
    pub compatibility: ::core::option::Option<Compatibility>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compatibility {
    #[prost(message, repeated, tag = "1")]
    pub active_devices: ::prost::alloc::vec::Vec<ActiveDevice>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveDevice {
    #[prost(string, optional, tag = "1")]
    pub required_os: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyLibrary {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub package_to_add: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub package_to_remove: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publisher {
    #[prost(message, optional, tag = "2")]
    pub publisher_stream: ::core::option::Option<PublisherStream>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublisherStream {
    #[prost(string, optional, tag = "3")]
    pub more_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditorChoice {
    #[prost(string, repeated, tag = "1")]
    pub bulletins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub stream: ::core::option::Option<SubStream>,
    #[prost(string, optional, tag = "4")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSet {
    #[prost(string, optional, tag = "1")]
    pub certificate_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub sha256: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dependencies {
    #[prost(int32, optional, tag = "1")]
    pub unknown: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub size: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "3")]
    pub dependency: ::prost::alloc::vec::Vec<Dependency>,
    #[prost(int32, optional, tag = "4")]
    pub target_sdk: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub unknown2: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "11")]
    pub split_apks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "13")]
    pub library_dependency: ::prost::alloc::vec::Vec<LibraryDependency>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dependency {
    #[prost(string, optional, tag = "1")]
    pub package_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub version: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub unknown4: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LibraryDependency {
    #[prost(string, optional, tag = "1")]
    pub package_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub version_code: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestingProgramInfo {
    #[prost(bool, optional, tag = "2")]
    pub subscribed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub subscribed_and_installed: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "5")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub image: ::core::option::Option<Image>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EarlyAccessInfo {
    #[prost(string, optional, tag = "3")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentDetails {
    #[prost(message, optional, tag = "1")]
    pub app_details: ::core::option::Option<AppDetails>,
    #[prost(message, optional, tag = "7")]
    pub subscription_details: ::core::option::Option<SubscriptionDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchDetails {
    #[prost(int32, optional, tag = "1")]
    pub base_version_code: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub size: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMetadata {
    #[prost(int32, optional, tag = "1")]
    pub file_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub version_code: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "3")]
    pub size: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "4")]
    pub split_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "5")]
    pub compressed_size: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "6")]
    pub patch_details: ::prost::alloc::vec::Vec<PatchDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionDetails {
    #[prost(int32, optional, tag = "1")]
    pub subscription_period: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    #[prost(bool, optional, tag = "2")]
    pub multi_corpus: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "3")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub icon_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub full_contents_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag = "6")]
    pub relevance: ::core::option::Option<f64>,
    #[prost(int64, optional, tag = "7")]
    pub estimated_results: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "8")]
    pub analytics_cookie: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub full_contents_list_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub next_page_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "11")]
    pub ordered: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResponse {
    #[prost(message, repeated, tag = "1")]
    pub bucket: ::prost::alloc::vec::Vec<Bucket>,
    #[prost(message, repeated, tag = "2")]
    pub item: ::prost::alloc::vec::Vec<Item>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub sub_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub category_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub creator: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub description_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "8")]
    pub offer: ::prost::alloc::vec::Vec<Offer>,
    #[prost(message, optional, tag = "9")]
    pub availability: ::core::option::Option<Availability>,
    #[prost(message, repeated, tag = "10")]
    pub image: ::prost::alloc::vec::Vec<Image>,
    #[prost(message, repeated, tag = "11")]
    pub sub_item: ::prost::alloc::vec::Vec<Item>,
    #[prost(message, optional, tag = "12")]
    pub container_metadata: ::core::option::Option<ContainerMetadata>,
    #[prost(message, optional, tag = "13")]
    pub details: ::core::option::Option<DocumentDetails>,
    #[prost(message, optional, tag = "14")]
    pub aggregate_rating: ::core::option::Option<AggregateRating>,
    #[prost(message, optional, tag = "15")]
    pub annotations: ::core::option::Option<Annotations>,
    #[prost(string, optional, tag = "16")]
    pub details_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub share_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub reviews_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "19")]
    pub backend_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "20")]
    pub purchase_details_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "21")]
    pub details_reusable: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "22")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "23")]
    pub translated_description_html: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(bytes = "vec", optional, tag = "24")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "25")]
    pub app_info: ::core::option::Option<AppInfo>,
    #[prost(bool, optional, tag = "26")]
    pub mature: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "27")]
    pub promotional_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "29")]
    pub available_for_preregistration: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "30")]
    pub tip: ::prost::alloc::vec::Vec<ReviewTip>,
    #[prost(string, optional, tag = "31")]
    pub review_snippets_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "32")]
    pub force_shareability: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "33")]
    pub use_wishlist_as_primary_action: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "34")]
    pub review_questions_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "39")]
    pub review_summary_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppInfo {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub section: ::prost::alloc::vec::Vec<AppInfoSection>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppInfoSection {
    #[prost(string, optional, tag = "1")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub container: ::core::option::Option<AppInfoContainer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppInfoContainer {
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<Image>,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotations {
    #[prost(message, optional, tag = "1")]
    pub section_related: ::core::option::Option<SectionMetaData>,
    #[prost(message, optional, tag = "2")]
    pub section_more_by: ::core::option::Option<SectionMetaData>,
    #[prost(message, repeated, tag = "4")]
    pub warning: ::prost::alloc::vec::Vec<Warning>,
    #[prost(message, optional, tag = "5")]
    pub section_body_of_work: ::core::option::Option<SectionMetaData>,
    #[prost(message, optional, tag = "6")]
    pub section_core_content: ::core::option::Option<SectionMetaData>,
    #[prost(message, optional, tag = "7")]
    pub overlay_meta_data: ::core::option::Option<OverlayMetaData>,
    #[prost(message, repeated, tag = "8")]
    pub badge_for_creator: ::prost::alloc::vec::Vec<Badge>,
    #[prost(message, repeated, tag = "9")]
    pub info_badge: ::prost::alloc::vec::Vec<Badge>,
    #[prost(message, optional, tag = "10")]
    pub annotation_link: ::core::option::Option<AnnotationLink>,
    #[prost(message, optional, tag = "11")]
    pub section_cross_sell: ::core::option::Option<SectionMetaData>,
    #[prost(message, optional, tag = "12")]
    pub section_related_item_type: ::core::option::Option<SectionMetaData>,
    #[prost(message, repeated, tag = "13")]
    pub promoted_doc: ::prost::alloc::vec::Vec<PromotedDoc>,
    #[prost(string, optional, tag = "14")]
    pub offer_note: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub privacy_policy_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "19")]
    pub suggestion_reasons: ::core::option::Option<SuggestionReasons>,
    #[prost(message, optional, tag = "20")]
    pub optimal_device_class_warning: ::core::option::Option<Warning>,
    #[prost(message, repeated, tag = "21")]
    pub badge_container: ::prost::alloc::vec::Vec<BadgeContainer>,
    #[prost(message, optional, tag = "22")]
    pub section_suggest_for_rating: ::core::option::Option<SectionMetaData>,
    #[prost(message, optional, tag = "24")]
    pub section_purchase_cross_sell: ::core::option::Option<SectionMetaData>,
    #[prost(message, repeated, tag = "25")]
    pub overflow_link: ::prost::alloc::vec::Vec<OverflowLink>,
    #[prost(string, optional, tag = "27")]
    pub attribution_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "28")]
    pub purchase_history_details: ::core::option::Option<PurchaseHistoryDetails>,
    #[prost(message, optional, tag = "29")]
    pub badge_for_legacy_rating: ::core::option::Option<Badge>,
    #[prost(message, repeated, tag = "30")]
    pub voucher_info: ::prost::alloc::vec::Vec<VoucherInfo>,
    #[prost(message, optional, tag = "32")]
    pub section_featured_apps: ::core::option::Option<SectionMetaData>,
    #[prost(message, repeated, tag = "34")]
    pub details_page_cluster: ::prost::alloc::vec::Vec<SectionMetaData>,
    #[prost(message, optional, tag = "35")]
    pub video_annotations: ::core::option::Option<VideoAnnotations>,
    #[prost(message, optional, tag = "36")]
    pub section_purchase_related_topics: ::core::option::Option<SectionMetaData>,
    #[prost(message, optional, tag = "37")]
    pub my_subscription_details: ::core::option::Option<MySubscriptionDetails>,
    #[prost(message, optional, tag = "38")]
    pub my_reward_details: ::core::option::Option<MyRewardDetails>,
    #[prost(message, repeated, tag = "39")]
    pub feature_badge: ::prost::alloc::vec::Vec<Badge>,
    #[prost(message, optional, tag = "42")]
    pub snippet: ::core::option::Option<Snippet>,
    #[prost(string, optional, tag = "48")]
    pub downloads_label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "50")]
    pub badge_for_rating: ::core::option::Option<Badge>,
    #[prost(message, optional, tag = "53")]
    pub category_info: ::core::option::Option<CategoryInfo>,
    #[prost(message, optional, tag = "60")]
    pub reasons: ::core::option::Option<EditorReason>,
    #[prost(message, optional, tag = "65")]
    pub top_chart_stream: ::core::option::Option<Stream>,
    #[prost(string, optional, tag = "66")]
    pub category_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "71")]
    pub chip: ::prost::alloc::vec::Vec<Chip>,
    #[prost(message, repeated, tag = "72")]
    pub display_badge: ::prost::alloc::vec::Vec<Badge>,
    #[prost(string, optional, tag = "80")]
    pub live_stream_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "85")]
    pub promotion_stream_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "91")]
    pub overlay_meta_data_extra: ::core::option::Option<OverlayMetaData>,
    #[prost(message, optional, tag = "94")]
    pub section_image: ::core::option::Option<SectionImage>,
    #[prost(message, optional, tag = "97")]
    pub category_stream: ::core::option::Option<SubStream>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditorReason {
    #[prost(string, repeated, tag = "1")]
    pub bulletin: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SectionMetaData {
    #[prost(string, optional, tag = "1")]
    pub header: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub list_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub browse_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverlayMetaData {
    #[prost(message, optional, tag = "1")]
    pub overlay_header: ::core::option::Option<OverlayHeader>,
    #[prost(message, optional, tag = "181")]
    pub overlay_title: ::core::option::Option<OverlayTitle>,
    #[prost(message, optional, tag = "182")]
    pub overlay_description: ::core::option::Option<OverlayDescription>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverlayHeader {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverlayTitle {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub composite_image: ::core::option::Option<CompositeImage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompositeImage {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "9")]
    pub type_alt: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "24")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "28")]
    pub url_alt: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverlayDescription {
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestionReasons {
    #[prost(message, repeated, tag = "2")]
    pub reason: ::prost::alloc::vec::Vec<Reason>,
    #[prost(message, optional, tag = "4")]
    pub neutral_dismissal: ::core::option::Option<Dismissal>,
    #[prost(message, optional, tag = "5")]
    pub positive_dismissal: ::core::option::Option<Dismissal>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reason {
    #[prost(string, optional, tag = "3")]
    pub description_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub reason_plus_one: ::core::option::Option<ReasonPlusOne>,
    #[prost(message, optional, tag = "5")]
    pub reason_review: ::core::option::Option<ReasonReview>,
    #[prost(message, optional, tag = "7")]
    pub dismissal: ::core::option::Option<Dismissal>,
    #[prost(message, optional, tag = "9")]
    pub reason_user_action: ::core::option::Option<ReasonUserAction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReasonPlusOne {
    #[prost(string, optional, tag = "1")]
    pub localized_description_html: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "3")]
    pub user_profile: ::prost::alloc::vec::Vec<UserProfile>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReasonReview {
    #[prost(message, optional, tag = "1")]
    pub review: ::core::option::Option<Review>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReasonUserAction {
    #[prost(message, optional, tag = "1")]
    pub user_profile: ::core::option::Option<UserProfile>,
    #[prost(string, optional, tag = "2")]
    pub localized_description_html: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dismissal {
    #[prost(string, optional, tag = "1")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub description_html: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snippet {
    #[prost(string, optional, tag = "1")]
    pub snippet_html: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyRewardDetails {
    #[prost(int64, optional, tag = "1")]
    pub expiration_time_millis: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub expiration_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub button_label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub link_action: ::core::option::Option<Link>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MySubscriptionDetails {
    #[prost(string, optional, tag = "1")]
    pub subscription_status_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub title_by_line_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub formatted_price: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub price_by_line_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub cancel_subscription: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "7")]
    pub payment_declined_learn_more_link: ::core::option::Option<Link>,
    #[prost(bool, optional, tag = "8")]
    pub in_trial_period: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "9")]
    pub title_by_line_icon: ::core::option::Option<Image>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnnotations {
    #[prost(bool, optional, tag = "1")]
    pub bundle: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub bundle_content_list_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub extras_content_list_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub also_available_in_list_url: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "5")]
    pub bundle_doc_id: ::prost::alloc::vec::Vec<DocId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoucherInfo {
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<Item>,
    #[prost(message, repeated, tag = "2")]
    pub offer: ::prost::alloc::vec::Vec<Offer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeContainer {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub image: ::prost::alloc::vec::Vec<Image>,
    #[prost(message, repeated, tag = "3")]
    pub badge: ::prost::alloc::vec::Vec<Badge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverflowLink {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub link: ::core::option::Option<Link>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotedDoc {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub image: ::prost::alloc::vec::Vec<Image>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub details_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Warning {
    #[prost(string, optional, tag = "1")]
    pub localized_message: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationLink {
    #[prost(string, optional, tag = "1")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub resolved_link: ::core::option::Option<ResolvedLink>,
    #[prost(int32, optional, tag = "3")]
    pub uri_backend: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rated {
    #[prost(string, optional, tag = "1")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<Image>,
    #[prost(string, optional, tag = "4")]
    pub learn_more_html_link: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Badge {
    #[prost(string, optional, tag = "1")]
    pub major: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<Image>,
    #[prost(string, optional, tag = "3")]
    pub minor: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub minor_html: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub sub_badge: ::core::option::Option<SubBadge>,
    #[prost(message, optional, tag = "7")]
    pub link: ::core::option::Option<StreamLink>,
    #[prost(string, optional, tag = "8")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "12")]
    pub stream: ::core::option::Option<SubStream>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubBadge {
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<Image>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub link: ::core::option::Option<StreamLink>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stream {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub stream: ::core::option::Option<SubStream>,
    #[prost(string, optional, tag = "3")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubStream {
    #[prost(message, optional, tag = "2")]
    pub link: ::core::option::Option<StreamLink>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Link {
    #[prost(string, optional, tag = "1")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub resolved_link: ::core::option::Option<ResolvedLink>,
    #[prost(int32, optional, tag = "3")]
    pub uri_backend: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamLink {
    #[prost(string, optional, tag = "1")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub stream_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub search_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub sub_category_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub search_query: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chip {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub stream: ::core::option::Option<SubStream>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryInfo {
    #[prost(string, optional, tag = "1")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub app_category: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedSubscriberInfo {
    #[prost(string, optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub encrypted_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub init_vector: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "5")]
    pub google_key_version: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub carrier_key_version: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Availability {
    #[prost(int32, optional, tag = "5")]
    pub restriction: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub offer_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "7")]
    pub rule: ::core::option::Option<Rule>,
    #[prost(group, repeated, tag = "9")]
    pub perdeviceavailabilityrestriction: ::prost::alloc::vec::Vec<
        availability::PerDeviceAvailabilityRestriction,
    >,
    #[prost(bool, optional, tag = "13")]
    pub available_if_owned: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "14")]
    pub install: ::prost::alloc::vec::Vec<Install>,
    #[prost(message, optional, tag = "16")]
    pub filter_info: ::core::option::Option<FilterEvaluationInfo>,
    #[prost(message, optional, tag = "17")]
    pub ownership_info: ::core::option::Option<OwnershipInfo>,
    #[prost(message, repeated, tag = "18")]
    pub availability_problem: ::prost::alloc::vec::Vec<AvailabilityProblem>,
    #[prost(bool, optional, tag = "21")]
    pub hidden: ::core::option::Option<bool>,
}
/// Nested message and enum types in `Availability`.
pub mod availability {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PerDeviceAvailabilityRestriction {
        #[prost(fixed64, optional, tag = "10")]
        pub android_id: ::core::option::Option<u64>,
        #[prost(int32, optional, tag = "11")]
        pub device_restriction: ::core::option::Option<i32>,
        #[prost(int64, optional, tag = "12")]
        pub channel_id: ::core::option::Option<i64>,
        #[prost(message, optional, tag = "15")]
        pub filter_info: ::core::option::Option<super::FilterEvaluationInfo>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailabilityProblem {
    #[prost(int32, optional, tag = "1")]
    pub problem_type: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "2")]
    pub missing_value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterEvaluationInfo {
    #[prost(message, repeated, tag = "1")]
    pub rule_evaluation: ::prost::alloc::vec::Vec<RuleEvaluation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    #[prost(bool, optional, tag = "1")]
    pub negate: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub operator: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub key: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "4")]
    pub string_arg: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, packed = "false", tag = "5")]
    pub long_arg: ::prost::alloc::vec::Vec<i64>,
    #[prost(double, repeated, packed = "false", tag = "6")]
    pub double_arg: ::prost::alloc::vec::Vec<f64>,
    #[prost(message, repeated, tag = "7")]
    pub sub_rule: ::prost::alloc::vec::Vec<Rule>,
    #[prost(int32, optional, tag = "8")]
    pub response_code: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "9")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(fixed64, repeated, packed = "false", tag = "10")]
    pub string_arg_hash: ::prost::alloc::vec::Vec<u64>,
    #[prost(int32, repeated, packed = "false", tag = "11")]
    pub const_arg: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "12")]
    pub availability_problem_type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "13")]
    pub include_missing_values: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleEvaluation {
    #[prost(message, optional, tag = "1")]
    pub rule: ::core::option::Option<Rule>,
    #[prost(string, repeated, tag = "2")]
    pub actual_string_value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub actual_long_value: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, repeated, packed = "false", tag = "4")]
    pub actual_bool_value: ::prost::alloc::vec::Vec<bool>,
    #[prost(double, repeated, packed = "false", tag = "5")]
    pub actual_double_value: ::prost::alloc::vec::Vec<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LibraryAppDetails {
    #[prost(string, optional, tag = "2")]
    pub certificate_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "3")]
    pub refund_timeout_timestamp: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "4")]
    pub post_delivery_refund_window_millis: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LibraryInAppDetails {
    #[prost(string, optional, tag = "1")]
    pub signed_purchase_data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub signature: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LibraryMutation {
    #[prost(message, optional, tag = "1")]
    pub doc_id: ::core::option::Option<DocId>,
    #[prost(int32, optional, tag = "2")]
    pub offer_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "3")]
    pub document_hash: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "4")]
    pub deleted: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "5")]
    pub app_details: ::core::option::Option<LibraryAppDetails>,
    #[prost(message, optional, tag = "6")]
    pub subscription_details: ::core::option::Option<LibrarySubscriptionDetails>,
    #[prost(message, optional, tag = "7")]
    pub in_app_details: ::core::option::Option<LibraryInAppDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LibrarySubscriptionDetails {
    #[prost(int64, optional, tag = "1")]
    pub initiation_timestamp: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub valid_until_timestamp: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "3")]
    pub auto_renewing: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "4")]
    pub trial_until_timestamp: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub signed_purchase_data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub signature: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LibraryUpdate {
    #[prost(int32, optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub corpus: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub server_token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "4")]
    pub mutation: ::prost::alloc::vec::Vec<LibraryMutation>,
    #[prost(bool, optional, tag = "5")]
    pub has_more: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "6")]
    pub library_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidAppNotificationData {
    #[prost(int32, optional, tag = "1")]
    pub version_code: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub asset_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InAppNotificationData {
    #[prost(string, optional, tag = "1")]
    pub checkout_order_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub in_app_notification_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LibraryDirtyData {
    #[prost(int32, optional, tag = "1")]
    pub backend: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub library_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    #[prost(int32, optional, tag = "1")]
    pub notification_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "3")]
    pub timestamp: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "4")]
    pub doc_id: ::core::option::Option<DocId>,
    #[prost(string, optional, tag = "5")]
    pub doc_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub user_email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub app_data: ::core::option::Option<AndroidAppNotificationData>,
    #[prost(message, optional, tag = "8")]
    pub app_delivery_data: ::core::option::Option<AndroidAppDeliveryData>,
    #[prost(message, optional, tag = "9")]
    pub purchase_removal_data: ::core::option::Option<PurchaseRemovalData>,
    #[prost(message, optional, tag = "10")]
    pub user_notification_data: ::core::option::Option<UserNotificationData>,
    #[prost(message, optional, tag = "11")]
    pub in_app_notification_data: ::core::option::Option<InAppNotificationData>,
    #[prost(message, optional, tag = "12")]
    pub purchase_declined_data: ::core::option::Option<PurchaseDeclinedData>,
    #[prost(string, optional, tag = "13")]
    pub notification_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "14")]
    pub library_update: ::core::option::Option<LibraryUpdate>,
    #[prost(message, optional, tag = "15")]
    pub library_dirty_data: ::core::option::Option<LibraryDirtyData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseDeclinedData {
    #[prost(int32, optional, tag = "1")]
    pub reason: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub show_notification: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseRemovalData {
    #[prost(bool, optional, tag = "1")]
    pub malicious: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserNotificationData {
    #[prost(string, optional, tag = "1")]
    pub notification_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub notification_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub ticker_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub dialog_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub dialog_text: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateRating {
    #[prost(int32, optional, tag = "1", default = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub star_rating: ::core::option::Option<f32>,
    #[prost(uint64, optional, tag = "3")]
    pub ratings_count: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub one_star_ratings: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "5")]
    pub two_star_ratings: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "6")]
    pub three_star_ratings: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "7")]
    pub four_star_ratings: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "8")]
    pub five_star_ratings: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "9")]
    pub thumbs_up_count: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "10")]
    pub thumbs_down_count: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "11")]
    pub comment_count: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "12")]
    pub bayesian_mean_rating: ::core::option::Option<f64>,
    #[prost(message, repeated, tag = "13")]
    pub tip: ::prost::alloc::vec::Vec<Tip>,
    #[prost(string, optional, tag = "17")]
    pub rating_label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub rating_count_label_abbreviated: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "19")]
    pub rating_count_label: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tip {
    #[prost(string, optional, tag = "1")]
    pub tip_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub polarity: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "4")]
    pub review_count: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub language: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub snippet_review_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewTip {
    #[prost(string, optional, tag = "1")]
    pub tip_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub polarity: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "4")]
    pub review_count: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptTosResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarrierBillingConfig {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub api_version: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub provisioning_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub credentials_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub tos_required: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub per_transaction_credentials_required: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub send_subscriber_id_with_carrier_billing_requests: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingConfig {
    #[prost(message, optional, tag = "1")]
    pub carrier_billing_config: ::core::option::Option<CarrierBillingConfig>,
    #[prost(int32, optional, tag = "2")]
    pub max_iab_api_version: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CorpusMetadata {
    #[prost(int32, optional, tag = "1")]
    pub backend: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub landing_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub library_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub recs_widget_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub shop_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Experiments {
    #[prost(string, repeated, tag = "1")]
    pub experiment_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfUpdateConfig {
    #[prost(int32, optional, tag = "1")]
    pub latest_client_version_code: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TocResponse {
    #[prost(message, repeated, tag = "1")]
    pub corpus: ::prost::alloc::vec::Vec<CorpusMetadata>,
    #[prost(int32, optional, tag = "2")]
    pub tos_version_deprecated: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub tos_content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub home_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub experiments: ::core::option::Option<Experiments>,
    #[prost(string, optional, tag = "6")]
    pub tos_checkbox_text_marketing_emails: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "7")]
    pub tos_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub user_settings: ::core::option::Option<UserSettings>,
    #[prost(string, optional, tag = "9")]
    pub icon_override_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "10")]
    pub self_update_config: ::core::option::Option<SelfUpdateConfig>,
    #[prost(bool, optional, tag = "11")]
    pub requires_upload_device_config: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "12")]
    pub billing_config: ::core::option::Option<BillingConfig>,
    #[prost(string, optional, tag = "13")]
    pub recs_widget_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub social_home_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "16")]
    pub age_verification_required: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "17")]
    pub g_plus_signup_enabled: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "18")]
    pub redeem_enabled: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "19")]
    pub help_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "20")]
    pub theme_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "21")]
    pub entertainment_home_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "22")]
    pub cookie: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSettings {
    #[prost(bool, optional, tag = "1")]
    pub tos_checkbox_marketing_emails_opted_in: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub privacy_setting: ::core::option::Option<PrivacySetting>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacySetting {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub current_status: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub enabled_by_default: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    #[prost(message, optional, tag = "1")]
    pub list_response: ::core::option::Option<ListResponse>,
    #[prost(message, optional, tag = "2")]
    pub details_response: ::core::option::Option<DetailsResponse>,
    #[prost(message, optional, tag = "3")]
    pub review_response: ::core::option::Option<ReviewResponse>,
    #[prost(message, optional, tag = "4")]
    pub buy_response: ::core::option::Option<BuyResponse>,
    #[prost(message, optional, tag = "5")]
    pub search_response: ::core::option::Option<SearchResponse>,
    #[prost(message, optional, tag = "6")]
    pub toc_response: ::core::option::Option<TocResponse>,
    #[prost(message, optional, tag = "7")]
    pub browse_response: ::core::option::Option<BrowseResponse>,
    #[prost(message, optional, tag = "8")]
    pub purchase_status_response: ::core::option::Option<PurchaseStatusResponse>,
    #[prost(string, optional, tag = "10")]
    pub log_response: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub flag_content_response: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "19")]
    pub bulk_details_response: ::core::option::Option<BulkDetailsResponse>,
    #[prost(message, optional, tag = "21")]
    pub delivery_response: ::core::option::Option<DeliveryResponse>,
    #[prost(message, optional, tag = "22")]
    pub accept_tos_response: ::core::option::Option<AcceptTosResponse>,
    #[prost(message, optional, tag = "24")]
    pub check_promo_offer_response: ::core::option::Option<CheckPromoOfferResponse>,
    #[prost(message, optional, tag = "25")]
    pub instrument_setup_info_response: ::core::option::Option<
        InstrumentSetupInfoResponse,
    >,
    #[prost(message, optional, tag = "26")]
    pub android_checkin_response: ::core::option::Option<AndroidCheckinResponse>,
    #[prost(message, optional, tag = "28")]
    pub upload_device_config_response: ::core::option::Option<
        UploadDeviceConfigResponse,
    >,
    #[prost(message, optional, tag = "40")]
    pub search_suggest_response: ::core::option::Option<SearchSuggestResponse>,
    #[prost(message, optional, tag = "30")]
    pub consume_purchase_response: ::core::option::Option<ConsumePurchaseResponse>,
    #[prost(message, optional, tag = "31")]
    pub billing_profile_response: ::core::option::Option<BillingProfileResponse>,
    #[prost(message, optional, tag = "34")]
    pub debug_settings_response: ::core::option::Option<DebugSettingsResponse>,
    #[prost(message, optional, tag = "35")]
    pub check_iab_promo_response: ::core::option::Option<CheckIabPromoResponse>,
    #[prost(message, optional, tag = "36")]
    pub user_activity_settings_response: ::core::option::Option<
        UserActivitySettingsResponse,
    >,
    #[prost(message, optional, tag = "37")]
    pub record_user_activity_response: ::core::option::Option<
        RecordUserActivityResponse,
    >,
    #[prost(message, optional, tag = "38")]
    pub redeem_code_response: ::core::option::Option<RedeemCodeResponse>,
    #[prost(message, optional, tag = "39")]
    pub self_update_response: ::core::option::Option<SelfUpdateResponse>,
    #[prost(message, optional, tag = "41")]
    pub get_initial_instrument_flow_state_response: ::core::option::Option<
        GetInitialInstrumentFlowStateResponse,
    >,
    #[prost(message, optional, tag = "42")]
    pub create_instrument_response: ::core::option::Option<CreateInstrumentResponse>,
    #[prost(message, optional, tag = "43")]
    pub challenge_response: ::core::option::Option<ChallengeResponse>,
    #[prost(message, optional, tag = "44")]
    pub backup_device_choices_response: ::core::option::Option<
        BackDeviceChoicesResponse,
    >,
    #[prost(message, optional, tag = "45")]
    pub backup_document_choices_response: ::core::option::Option<
        BackupDocumentChoicesResponse,
    >,
    #[prost(message, optional, tag = "46")]
    pub early_update_response: ::core::option::Option<EarlyUpdateResponse>,
    #[prost(message, optional, tag = "47")]
    pub preloads_response: ::core::option::Option<PreloadsResponse>,
    #[prost(message, optional, tag = "48")]
    pub my_accounts_response: ::core::option::Option<MyAccountsResponse>,
    #[prost(message, optional, tag = "49")]
    pub content_filter_response: ::core::option::Option<ContentFilterResponse>,
    #[prost(message, optional, tag = "50")]
    pub experiments_response: ::core::option::Option<ExperimentsResponse>,
    #[prost(message, optional, tag = "51")]
    pub survey_response: ::core::option::Option<SurveyResponse>,
    #[prost(message, optional, tag = "52")]
    pub ping_response: ::core::option::Option<PingResponse>,
    #[prost(message, optional, tag = "53")]
    pub update_user_setting_response: ::core::option::Option<UpdateUserSettingResponse>,
    #[prost(message, optional, tag = "54")]
    pub get_user_settings_response: ::core::option::Option<GetUserSettingsResponse>,
    #[prost(message, optional, tag = "56")]
    pub get_sharing_settings_response: ::core::option::Option<
        GetSharingSettingsResponse,
    >,
    #[prost(message, optional, tag = "57")]
    pub update_sharing_settings_response: ::core::option::Option<
        UpdateSharingSettingsResponse,
    >,
    #[prost(message, optional, tag = "58")]
    pub review_snippets_response: ::core::option::Option<ReviewSnippetsResponse>,
    #[prost(message, optional, tag = "59")]
    pub document_sharing_state_response: ::core::option::Option<
        DocumentSharingStateResponse,
    >,
    #[prost(message, optional, tag = "70")]
    pub module_delivery_response: ::core::option::Option<ModuleDeliveryResponse>,
    #[prost(message, optional, tag = "80")]
    pub testing_program_response: ::core::option::Option<TestingProgramResponse>,
    #[prost(message, optional, tag = "129")]
    pub review_summary_response: ::core::option::Option<ReviewResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckIabPromoResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserActivitySettingsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordUserActivityResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemCodeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfUpdateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInitialInstrumentFlowStateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstrumentResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChallengeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackDeviceChoicesResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupDocumentChoicesResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EarlyUpdateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreloadsResponse {
    #[prost(message, optional, tag = "1")]
    pub config_preload: ::core::option::Option<preloads_response::Preload>,
    #[prost(message, repeated, tag = "2")]
    pub app_preload: ::prost::alloc::vec::Vec<preloads_response::Preload>,
}
/// Nested message and enum types in `PreloadsResponse`.
pub mod preloads_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Preload {
        #[prost(message, optional, tag = "1")]
        pub doc_id: ::core::option::Option<super::DocId>,
        #[prost(int32, optional, tag = "2")]
        pub version_code: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "3")]
        pub title: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "4")]
        pub icon: ::core::option::Option<super::Image>,
        #[prost(string, optional, tag = "5")]
        pub delivery_token: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "6")]
        pub install_location: ::core::option::Option<i32>,
        #[prost(int64, optional, tag = "7")]
        pub size: ::core::option::Option<i64>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyAccountsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentFilterResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SurveyResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserSettingResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserSettingsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSharingSettingsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSharingSettingsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewSnippetsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentSharingStateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDeliveryResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreFetch {
    #[prost(string, optional, tag = "1")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub response: ::core::option::Option<ResponseWrapper>,
    #[prost(string, optional, tag = "3")]
    pub etag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub ttl: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "5")]
    pub soft_ttl: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMetadata {
    #[prost(int64, optional, tag = "1")]
    pub latency_millis: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Targets {
    #[prost(int64, repeated, packed = "false", tag = "1")]
    pub target_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCookie {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCookies {
    #[prost(message, repeated, tag = "1")]
    pub server_cookie: ::prost::alloc::vec::Vec<ServerCookie>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseWrapper {
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<Payload>,
    #[prost(message, optional, tag = "2")]
    pub commands: ::core::option::Option<ServerCommands>,
    #[prost(message, repeated, tag = "3")]
    pub pre_fetch: ::prost::alloc::vec::Vec<PreFetch>,
    #[prost(message, repeated, tag = "4")]
    pub notification: ::prost::alloc::vec::Vec<Notification>,
    #[prost(message, optional, tag = "5")]
    pub server_metadata: ::core::option::Option<ServerMetadata>,
    #[prost(message, optional, tag = "6")]
    pub targets: ::core::option::Option<Targets>,
    #[prost(message, optional, tag = "7")]
    pub server_cookies: ::core::option::Option<ServerCookies>,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseWrapperApi {
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<PayloadApi>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadApi {
    #[prost(message, optional, tag = "5")]
    pub user_profile_response: ::core::option::Option<UserProfileResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProfileResponse {
    #[prost(message, optional, tag = "1")]
    pub user_profile: ::core::option::Option<UserProfile>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCommands {
    #[prost(bool, optional, tag = "1")]
    pub clear_cache: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub display_error_message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub log_error_stacktrace: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReviewsResponse {
    #[prost(message, repeated, tag = "1")]
    pub review: ::prost::alloc::vec::Vec<Review>,
    #[prost(int64, optional, tag = "2")]
    pub matching_count: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Review {
    #[prost(string, optional, tag = "1")]
    pub author_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "5")]
    pub timestamp: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "6")]
    pub star_rating: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub comment_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "19")]
    pub device_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "29")]
    pub reply_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "30")]
    pub reply_time_stamp: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "31")]
    pub author: ::core::option::Option<ReviewAuthor>,
    #[prost(message, optional, tag = "33")]
    pub user_profile: ::core::option::Option<UserProfile>,
    #[prost(message, optional, tag = "34")]
    pub sentiment: ::core::option::Option<Image>,
    #[prost(int32, optional, tag = "35")]
    pub helpful_count: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "38")]
    pub thumbs_up_count: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriticReviewsResponse {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<Image>,
    #[prost(uint32, optional, tag = "3")]
    pub total_num_reviews: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub percent_favorable: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub source_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub source: ::core::option::Option<Link>,
    #[prost(message, repeated, tag = "7")]
    pub review: ::prost::alloc::vec::Vec<Review>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewAuthor {
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub avatar: ::core::option::Option<Image>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProfile {
    #[prost(string, optional, tag = "1")]
    pub profile_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub person_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub profile_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub person_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "10")]
    pub image: ::prost::alloc::vec::Vec<Image>,
    #[prost(string, optional, tag = "19")]
    pub profile_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "22")]
    pub profile_description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewResponse {
    #[prost(message, optional, tag = "1")]
    pub user_reviews_response: ::core::option::Option<GetReviewsResponse>,
    #[prost(string, optional, tag = "2")]
    pub next_page_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub user_review: ::core::option::Option<Review>,
    #[prost(string, optional, tag = "4")]
    pub suggestions_list_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub critic_reviews_response: ::core::option::Option<CriticReviewsResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedSearch {
    #[prost(string, optional, tag = "1")]
    pub search_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub header: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub backend_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4", default = "1")]
    pub doc_type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub current: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    #[prost(string, optional, tag = "1")]
    pub original_query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub suggested_query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub aggregate_query: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "4")]
    pub bucket: ::prost::alloc::vec::Vec<Bucket>,
    #[prost(message, repeated, tag = "5")]
    pub item: ::prost::alloc::vec::Vec<Item>,
    #[prost(message, repeated, tag = "6")]
    pub related_search: ::prost::alloc::vec::Vec<RelatedSearch>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub server_logs_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "8")]
    pub full_page_replaced: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "9")]
    pub contains_snow: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "10")]
    pub next_page_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSuggestResponse {
    #[prost(message, repeated, tag = "1")]
    pub entry: ::prost::alloc::vec::Vec<SearchSuggestEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSuggestEntry {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub suggested_query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub image_container: ::core::option::Option<search_suggest_entry::ImageContainer>,
    #[prost(string, optional, tag = "6")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub package_name_container: ::core::option::Option<
        search_suggest_entry::PackageNameContainer,
    >,
}
/// Nested message and enum types in `SearchSuggestEntry`.
pub mod search_suggest_entry {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImageContainer {
        #[prost(string, optional, tag = "5")]
        pub image_url: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PackageNameContainer {
        #[prost(string, optional, tag = "1")]
        pub package_name: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestingProgramResponse {
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<TestingProgramResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestingProgramResult {
    #[prost(message, optional, tag = "4")]
    pub details: ::core::option::Option<TestingProgramDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestingProgramDetails {
    #[prost(bool, optional, tag = "2")]
    pub subscribed: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "3")]
    pub id: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "4")]
    pub unsubscribed: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogRequest {
    #[prost(int64, optional, tag = "1")]
    pub timestamp: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub download_confirmation_query: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestingProgramRequest {
    #[prost(string, optional, tag = "1")]
    pub package_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub subscribe: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadDeviceConfigRequest {
    #[prost(message, optional, tag = "1")]
    pub device_configuration: ::core::option::Option<DeviceConfigurationProto>,
    #[prost(string, optional, tag = "2")]
    pub manufacturer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub gcm_registration_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadDeviceConfigResponse {
    #[prost(string, optional, tag = "1")]
    pub upload_device_config_token: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidCheckinRequest {
    #[prost(string, optional, tag = "1")]
    pub imei: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2", default = "0")]
    pub id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub digest: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub checkin: ::core::option::Option<AndroidCheckinProto>,
    #[prost(string, optional, tag = "5")]
    pub desired_build: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub locale: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "7")]
    pub logging_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "8")]
    pub market_checkin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "9")]
    pub mac_addr: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub meid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "11")]
    pub account_cookie: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub time_zone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(fixed64, optional, tag = "13")]
    pub security_token: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "14")]
    pub version: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "15")]
    pub ota_cert: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub serial_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub esn: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "18")]
    pub device_configuration: ::core::option::Option<DeviceConfigurationProto>,
    #[prost(string, repeated, tag = "19")]
    pub mac_addr_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "20")]
    pub fragment: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "21")]
    pub user_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "22")]
    pub user_serial_number: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidCheckinResponse {
    #[prost(bool, optional, tag = "1")]
    pub stats_ok: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub intent: ::prost::alloc::vec::Vec<AndroidIntentProto>,
    #[prost(int64, optional, tag = "3")]
    pub time_msec: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "4")]
    pub digest: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub setting: ::prost::alloc::vec::Vec<GservicesSetting>,
    #[prost(bool, optional, tag = "6")]
    pub market_ok: ::core::option::Option<bool>,
    #[prost(fixed64, optional, tag = "7")]
    pub android_id: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "8")]
    pub security_token: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "9")]
    pub settings_diff: ::core::option::Option<bool>,
    #[prost(string, repeated, tag = "10")]
    pub delete_setting: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub device_checkin_consistency_token: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GservicesSetting {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidBuildProto {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub product: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub carrier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub radio: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub bootloader: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub client: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "7")]
    pub timestamp: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "8")]
    pub google_services: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "9")]
    pub device: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "10")]
    pub sdk_version: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "11")]
    pub model: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub manufacturer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub build_product: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "14")]
    pub ota_installed: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidCheckinProto {
    #[prost(message, optional, tag = "1")]
    pub build: ::core::option::Option<AndroidBuildProto>,
    #[prost(int64, optional, tag = "2")]
    pub last_checkin_msec: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "3")]
    pub event: ::prost::alloc::vec::Vec<AndroidEventProto>,
    #[prost(message, repeated, tag = "4")]
    pub stat: ::prost::alloc::vec::Vec<AndroidStatisticProto>,
    #[prost(string, repeated, tag = "5")]
    pub requested_group: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub cell_operator: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub sim_operator: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub roaming: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "9")]
    pub user_number: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidEventProto {
    #[prost(string, optional, tag = "1")]
    pub tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "3")]
    pub time_msec: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidIntentProto {
    #[prost(string, optional, tag = "1")]
    pub action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub data_uri: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub mime_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub java_class: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(group, repeated, tag = "5")]
    pub extra: ::prost::alloc::vec::Vec<android_intent_proto::Extra>,
}
/// Nested message and enum types in `AndroidIntentProto`.
pub mod android_intent_proto {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Extra {
        #[prost(string, optional, tag = "6")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "7")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidStatisticProto {
    #[prost(string, optional, tag = "1")]
    pub tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub count: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub sum: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientLibraryState {
    #[prost(int32, optional, tag = "1")]
    pub corpus: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub server_token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag = "3")]
    pub hash_code_sum: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub library_size: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub library_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidDataUsageProto {
    #[prost(int32, optional, tag = "1")]
    pub version: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub current_report_msec: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "3")]
    pub key_to_package_name_mapping: ::prost::alloc::vec::Vec<KeyToPackageNameMapping>,
    #[prost(message, repeated, tag = "4")]
    pub payload_level_app_stat: ::prost::alloc::vec::Vec<PayloadLevelAppStat>,
    #[prost(message, repeated, tag = "5")]
    pub ip_layer_network_stat: ::prost::alloc::vec::Vec<IpLayerNetworkStat>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidUsageStatsReport {
    #[prost(int64, optional, tag = "1")]
    pub android_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub logging_id: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "3")]
    pub usage_stats: ::core::option::Option<UsageStatsExtensionProto>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppBucket {
    #[prost(int64, optional, tag = "1")]
    pub bucket_start_msec: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub bucket_duration_msec: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "3")]
    pub stat_counters: ::prost::alloc::vec::Vec<StatCounters>,
    #[prost(int64, optional, tag = "4")]
    pub operation_count: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CounterData {
    #[prost(int64, optional, tag = "1")]
    pub bytes: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub packets: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpLayerAppStat {
    #[prost(int32, optional, tag = "1")]
    pub package_key: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub application_tag: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub ip_layer_app_bucket: ::prost::alloc::vec::Vec<AppBucket>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpLayerNetworkBucket {
    #[prost(int64, optional, tag = "1")]
    pub bucket_start_msec: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub bucket_duration_msec: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "3")]
    pub stat_counters: ::prost::alloc::vec::Vec<StatCounters>,
    #[prost(int64, optional, tag = "4")]
    pub network_active_duration: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpLayerNetworkStat {
    #[prost(string, optional, tag = "1")]
    pub network_details: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub ip_layer_network_bucket: ::prost::alloc::vec::Vec<IpLayerNetworkBucket>,
    #[prost(message, repeated, tag = "4")]
    pub ip_layer_app_stat: ::prost::alloc::vec::Vec<IpLayerAppStat>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyToPackageNameMapping {
    #[prost(int32, optional, tag = "1")]
    pub package_key: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub uid_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub shared_package: ::prost::alloc::vec::Vec<PackageInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageInfo {
    #[prost(string, optional, tag = "1")]
    pub pkg_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub version_code: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadLevelAppStat {
    #[prost(int32, optional, tag = "1")]
    pub package_key: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub application_tag: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub payload_level_app_bucket: ::prost::alloc::vec::Vec<AppBucket>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatCounters {
    #[prost(int32, optional, tag = "1")]
    pub network_proto: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub direction: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub counter_data: ::core::option::Option<CounterData>,
    #[prost(int32, optional, tag = "4")]
    pub fg_bg: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageStatsExtensionProto {
    #[prost(message, optional, tag = "1")]
    pub data_usage: ::core::option::Option<AndroidDataUsageProto>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyLibraryRequest {
    #[prost(string, optional, tag = "1")]
    pub library_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub add_package_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub remove_package_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
