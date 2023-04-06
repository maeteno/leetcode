package com.maeteno.leetcode;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import lombok.Builder;
import lombok.Data;
import lombok.extern.slf4j.Slf4j;

import java.util.ArrayList;
import java.util.List;

@Slf4j
public class StrDiff {
    private final ObjectMapper mapper = new ObjectMapper();

    /**
     * 最长公共子串
     */
    public String lcs(String s1, String s2) {
        List<Sub> sub1 = substring(s1);
        List<Sub> sub2 = substring(s2);

        String subText = "";
        for (Sub sub : sub1) {
            for (Sub sub3 : sub2) {
                if (sub.sub.equals(sub3.sub) && (sub.sub.length() > subText.length())) {
                    subText = sub.sub;
                }
            }
        }

        return subText;
    }

    public List<Diff> diff1(String s1, String s2) {
        List<Sub> sub1 = substring(s1);
        List<Sub> sub2 = substring(s2);

        List<Diff> diffs = new ArrayList<>();

        int index = -1;
        int subLength = 0;
        String subText = "";
        for (Sub sub : sub1) {
            for (Sub sub3 : sub2) {
                if (sub.sub.equals(sub3.sub)) {
                    log.info("{}:{} => {}:{}", sub.start, sub3.start, sub.sub, sub3.sub);
                    if (index == -1) {
                        index = sub.start;
                    }
//
//                    if (sub.sub.length() > subLength) {
//                        if (index == sub.start) {
//                            subText = subText;
//                        }
//                    }
                }
            }
        }


        return null;
    }

    public String toJson(Object obj) {
        try {
            return mapper.writeValueAsString(obj);
        } catch (JsonProcessingException e) {
            return obj.toString();
        }
    }

    public List<Sub> substring(String str) {
        List<Sub> subList = new ArrayList<>();
        for (int i = 0; i < str.length(); i++) {
            for (int j = i + 1; j <= str.length(); j++) {
                subList.add(Sub.builder()
                        .start(i)
                        .sub(str.substring(i, j))
                        .build());
            }
        }

        return subList;
    }


    @Data
    @Builder
    public static class Diff {
        private Option option;
        private String text;
    }

    public enum Option {
        EQUAL, DELETE, INSERT
    }

    @Data
    @Builder
    public static class Sub {
        private int start;
        private String sub;
    }

}
