package com.maeteno.leetcode;

import lombok.extern.slf4j.Slf4j;
import org.junit.Assert;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.mockito.runners.MockitoJUnitRunner;

import java.util.List;

@Slf4j
@RunWith(MockitoJUnitRunner.class)
public class StrDiffTest {
    private final StrDiff strDiff = new StrDiff();

    @Test
    public void test() {
        List<StrDiff.Diff> diffs = strDiff.diff1("ABCDEF", "ABCEFG");

        log.info("{}", diffs);

        Assert.assertTrue(true);
    }

    @Test
    public void test_2() {
        List<StrDiff.Diff> diffs = strDiff.diff1("ABCDEF", "ABCDEF");

        log.info("{}", diffs);

        Assert.assertTrue(true);
    }

    @Test
    public void test_lcs_1() {
        String sub = strDiff.lcs("ABCDEF", "ABCEFG");
        log.info("{}", sub);
        Assert.assertTrue(true);
    }

    @Test
    public void test_lcs_2() {
        String sub = strDiff.lcs("GGABCDEFDF", "HGABCDEFGTH");
        log.info("{}", sub);
        Assert.assertTrue(true);
    }
}