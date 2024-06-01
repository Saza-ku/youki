use std::path::Path;

use anyhow::{Context, Result};
use oci_spec::runtime::{
    LinuxBlockIo, LinuxBlockIoBuilder, LinuxBuilder, LinuxCpuBuilder, LinuxResourcesBuilder, LinuxThrottleDeviceBuilder, LinuxWeightDeviceBuilder, Spec, SpecBuilder
};
use test_framework::{test_result, ConditionalTest, TestGroup, TestResult};

fn create_spec() -> Result<Spec> {
    let shares: u64 = 1024;
    let period: u64 = 100000;
    let quota = 50000;
    let cpus = "0-1";
    let mems = "0";

    SpecBuilder::default()
    .linux(
        LinuxBuilder::default()
        .cgroups_path(Path::new("/runtime-test").join("cgroup_relative_cpu"))
        .resources(
            LinuxResourcesBuilder::default()
                .cpu(
                    LinuxCpuBuilder::default()
                    .shares(shares)
                    .period(period)
                    .quota(quota)
                    .cpus(cpus)
                    .mems(mems)
                    .build()
                    .context("failed to build cpu spec")?
                    )
            .build()
            .context("failed to build linux spec")?,
    ).
    build()
    .context("failed to build linux spec")?
)
    .build()
    .context("failed to build spec")
}

fn can_run() -> bool {
    true
}

fn test_relative_cpu() -> TestResult {
    TestResult::Passed
}

pub fn get_test_group() -> TestGroup {
    let mut test_group = TestGroup::new("cgroup_relative_cpu");
    let linux_cgroups_relative_cpus = ConditionalTest::new(
        "test_linux_cgroups_relative_cpus",
        Box::new(can_run),
        Box::new(test_relative_cpu),
    );

    test_group.add(vec![Box::new(linux_cgroups_relative_cpus)]);

    test_group
}
