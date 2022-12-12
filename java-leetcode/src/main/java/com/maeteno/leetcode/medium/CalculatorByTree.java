package com.maeteno.leetcode.medium;

import lombok.extern.slf4j.Slf4j;

/**
 * <a href="https://leetcode.cn/problems/calculator-lcci/">面试题 16.26. 计算器</a>
 * 给定一个包含正整数、加(+)、减(-)、乘(*)、除(/)的算数表达式(括号除外)，计算其结果。
 * <p>
 * 表达式仅包含非负整数，+， - ，*，/ 四种运算符和空格  。 整数除法仅保留整数部分。
 *
 * @author SL.F
 */
@Slf4j
public class CalculatorByTree {
    public int calculate(String str) {
        char[] chars = str.toCharArray();

        StringBuilder cache = new StringBuilder();
        Node root = null;
        for (char c : chars) {
            cache = switch (c) {
                case ' ' -> cache;
                case '+', '-', '*', '/' -> {
                    var valNode = new Node();
                    valNode.type = TypeEnum.VALUE;
                    valNode.vale = cache.toString();

                    var opNode = new Node();
                    opNode.type = TypeEnum.OPERATE;
                    opNode.vale = String.valueOf(c);

                    if (root == null) {
                        opNode.right = valNode;
                        root = opNode;
                    } else {
                        root.right = valNode;
                        opNode.left = root;
                        root = opNode;
                    }

                    yield new StringBuilder();
                }
                default -> cache.append(c);
            };
        }

        if (root != null) {
            var valNode = new Node();
            valNode.type = TypeEnum.VALUE;
            valNode.vale = cache.toString();
            root.right = valNode;
        }

        log.info("Root: {}", root);
        return str.length();
    }

    private static class Node {
        private TypeEnum type;
        private String vale;
        private Node left;
        private Node right;

        @Override
        public String toString() {
            return String.format("{\"type\":\"%s\",\"vale\":\"%s\",\"left\":%s,\"right\":%s}", type, vale, left, right);
        }
    }

    private enum TypeEnum {
        /**
         * 操作
         */
        OPERATE,
        /**
         * 值
         */
        VALUE
    }
}
