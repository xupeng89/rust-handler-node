use sea_orm_migration::prelude::*;
#[derive(Iden)]
pub enum BinaryTables {
    #[iden = "model_fluid_package_binary_nrtl_entity"]
    Nrtl,
    #[iden = "model_fluid_package_binary_nrtl_rk_entity"]
    NrtlRk,
    #[iden = "model_fluid_package_binary_pr_entity"]
    Pr,
    #[iden = "model_fluid_package_binary_psrk_entity"]
    Psrk,
    #[iden = "model_fluid_package_binary_rk_entity"]
    Rk,
    #[iden = "model_fluid_package_binary_srk_entity"]
    Srk,
    #[iden = "model_fluid_package_binary_uniquac_entity"]
    Uniquac,
    #[iden = "model_fluid_package_binary_wilsion_entity"]
    Wilson,
}

#[derive(Iden)]
pub enum BinaryCols {
    Id,
    FluidPackageId,
    CompoundI,
    CompoundIName,
    CompoundJ,
    CompoundJName,
    #[iden = "AIJ"]
    Aij,
    #[iden = "AJI"]
    Aji,
    #[iden = "BIJ"]
    Bij,
    #[iden = "BJI"]
    Bji,
    #[iden = "CIJ"]
    Cij,
    #[iden = "CJI"]
    Cji,
    #[iden = "DIJ"]
    Dij,
    #[iden = "DJI"]
    Dji,
    #[iden = "EIJ"]
    Eij,
    #[iden = "EJI"]
    Eji,
    #[iden = "FIJ"]
    Fij,
    #[iden = "FJI"]
    Fji,
    #[iden = "KAIJ"]
    Kaij,
    #[iden = "KBIJ"]
    Kbij,
    #[iden = "KCIJ"]
    Kcij,
    #[iden = "TIJ"]
    Tij,
    #[iden = "TJI"]
    Tji,
    #[iden = "VIJ"]
    Vij,
    #[iden = "VJI"]
    Vji,
    MinT,
    MaxT,
    IsDefault,
    IsDefaultId,
}
